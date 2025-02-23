//! See [`PathTransform`].

use crate::helpers::mod_path_to_ast;
use either::Either;
use hir::{AsAssocItem, HirDisplay, SemanticsScope};
use crablangc_hash::FxHashMap;
use syntax::{
    ast::{self, AstNode},
    ted, SyntaxNode,
};

/// `PathTransform` substitutes path in SyntaxNodes in bulk.
///
/// This is mostly useful for IDE code generation. If you paste some existing
/// code into a new context (for example, to add method overrides to an `impl`
/// block), you generally want to appropriately qualify the names, and sometimes
/// you might want to substitute generic parameters as well:
///
/// ```
/// mod x {
///   pub struct A<V>;
///   pub trait T<U> { fn foo(&self, _: U) -> A<U>; }
/// }
///
/// mod y {
///   use x::T;
///
///   impl T<()> for () {
///      // If we invoke **Add Missing Members** here, we want to copy-paste `foo`.
///      // But we want a slightly-modified version of it:
///      fn foo(&self, _: ()) -> x::A<()> {}
///   }
/// }
/// ```
pub struct PathTransform<'a> {
    generic_def: Option<hir::GenericDef>,
    substs: Vec<ast::Type>,
    target_scope: &'a SemanticsScope<'a>,
    source_scope: &'a SemanticsScope<'a>,
}

impl<'a> PathTransform<'a> {
    pub fn trait_impl(
        target_scope: &'a SemanticsScope<'a>,
        source_scope: &'a SemanticsScope<'a>,
        trait_: hir::Trait,
        impl_: ast::Impl,
    ) -> PathTransform<'a> {
        PathTransform {
            source_scope,
            target_scope,
            generic_def: Some(trait_.into()),
            substs: get_syntactic_substs(impl_).unwrap_or_default(),
        }
    }

    pub fn function_call(
        target_scope: &'a SemanticsScope<'a>,
        source_scope: &'a SemanticsScope<'a>,
        function: hir::Function,
        generic_arg_list: ast::GenericArgList,
    ) -> PathTransform<'a> {
        PathTransform {
            source_scope,
            target_scope,
            generic_def: Some(function.into()),
            substs: get_type_args_from_arg_list(generic_arg_list).unwrap_or_default(),
        }
    }

    pub fn generic_transformation(
        target_scope: &'a SemanticsScope<'a>,
        source_scope: &'a SemanticsScope<'a>,
    ) -> PathTransform<'a> {
        PathTransform { source_scope, target_scope, generic_def: None, substs: Vec::new() }
    }

    pub fn apply(&self, syntax: &SyntaxNode) {
        self.build_ctx().apply(syntax)
    }

    pub fn apply_all<'b>(&self, nodes: impl IntoIterator<Item = &'b SyntaxNode>) {
        let ctx = self.build_ctx();
        for node in nodes {
            ctx.apply(node);
        }
    }

    fn build_ctx(&self) -> Ctx<'a> {
        let db = self.source_scope.db;
        let target_module = self.target_scope.module();
        let source_module = self.source_scope.module();
        let skip = match self.generic_def {
            // this is a trait impl, so we need to skip the first type parameter -- this is a bit hacky
            Some(hir::GenericDef::Trait(_)) => 1,
            _ => 0,
        };
        let substs_by_param: FxHashMap<_, _> = self
            .generic_def
            .into_iter()
            .flat_map(|it| it.type_params(db))
            .skip(skip)
            // The actual list of trait type parameters may be longer than the one
            // used in the `impl` block due to trailing default type parameters.
            // For that case we extend the `substs` with an empty iterator so we
            // can still hit those trailing values and check if they actually have
            // a default type. If they do, go for that type from `hir` to `ast` so
            // the resulting change can be applied correctly.
            .zip(self.substs.iter().map(Some).chain(std::iter::repeat(None)))
            .filter_map(|(k, v)| match k.split(db) {
                Either::Left(_) => None,
                Either::Right(t) => match v {
                    Some(v) => Some((k, v.clone())),
                    None => {
                        let default = t.default(db)?;
                        Some((
                            k,
                            ast::make::ty(
                                &default.display_source_code(db, source_module.into()).ok()?,
                            ),
                        ))
                    }
                },
            })
            .collect();
        Ctx { substs: substs_by_param, target_module, source_scope: self.source_scope }
    }
}

struct Ctx<'a> {
    substs: FxHashMap<hir::TypeOrConstParam, ast::Type>,
    target_module: hir::Module,
    source_scope: &'a SemanticsScope<'a>,
}

impl<'a> Ctx<'a> {
    fn apply(&self, item: &SyntaxNode) {
        // `transform_path` may update a node's parent and that would break the
        // tree traversal. Thus all paths in the tree are collected into a vec
        // so that such operation is safe.
        let paths = item
            .preorder()
            .filter_map(|event| match event {
                syntax::WalkEvent::Enter(_) => None,
                syntax::WalkEvent::Leave(node) => Some(node),
            })
            .filter_map(ast::Path::cast)
            .collect::<Vec<_>>();

        for path in paths {
            self.transform_path(path);
        }
    }
    fn transform_path(&self, path: ast::Path) -> Option<()> {
        if path.qualifier().is_some() {
            return None;
        }
        if path.segment().map_or(false, |s| {
            s.param_list().is_some() || (s.self_token().is_some() && path.parent_path().is_none())
        }) {
            // don't try to qualify `Fn(Foo) -> Bar` paths, they are in prelude anyway
            // don't try to qualify sole `self` either, they are usually locals, but are returned as modules due to namespace clashing
            return None;
        }

        let resolution = self.source_scope.speculative_resolve(&path)?;

        match resolution {
            hir::PathResolution::TypeParam(tp) => {
                if let Some(subst) = self.substs.get(&tp.merge()) {
                    let parent = path.syntax().parent()?;
                    if let Some(parent) = ast::Path::cast(parent.clone()) {
                        // Path inside path means that there is an associated
                        // type/constant on the type parameter. It is necessary
                        // to fully qualify the type with `as Trait`. Even
                        // though it might be unnecessary if `subst` is generic
                        // type, always fully qualifying the path is safer
                        // because of potential clash of associated types from
                        // multiple traits

                        let trait_ref = find_trait_for_assoc_item(
                            self.source_scope,
                            tp,
                            parent.segment()?.name_ref()?,
                        )
                        .and_then(|trait_ref| {
                            let found_path = self.target_module.find_use_path(
                                self.source_scope.db.upcast(),
                                hir::ModuleDef::Trait(trait_ref),
                                false,
                            )?;
                            match ast::make::ty_path(mod_path_to_ast(&found_path)) {
                                ast::Type::PathType(path_ty) => Some(path_ty),
                                _ => None,
                            }
                        });

                        let segment = ast::make::path_segment_ty(subst.clone(), trait_ref);
                        let qualified =
                            ast::make::path_from_segments(std::iter::once(segment), false);
                        ted::replace(path.syntax(), qualified.clone_for_update().syntax());
                    } else if let Some(path_ty) = ast::PathType::cast(parent) {
                        ted::replace(
                            path_ty.syntax(),
                            subst.clone_subtree().clone_for_update().syntax(),
                        );
                    } else {
                        ted::replace(
                            path.syntax(),
                            subst.clone_subtree().clone_for_update().syntax(),
                        );
                    }
                }
            }
            hir::PathResolution::Def(def) if def.as_assoc_item(self.source_scope.db).is_none() => {
                if let hir::ModuleDef::Trait(_) = def {
                    if matches!(path.segment()?.kind()?, ast::PathSegmentKind::Type { .. }) {
                        // `speculative_resolve` resolves segments like `<T as
                        // Trait>` into `Trait`, but just the trait name should
                        // not be used as the replacement of the original
                        // segment.
                        return None;
                    }
                }

                let found_path =
                    self.target_module.find_use_path(self.source_scope.db.upcast(), def, false)?;
                let res = mod_path_to_ast(&found_path).clone_for_update();
                if let Some(args) = path.segment().and_then(|it| it.generic_arg_list()) {
                    if let Some(segment) = res.segment() {
                        let old = segment.get_or_create_generic_arg_list();
                        ted::replace(old.syntax(), args.clone_subtree().syntax().clone_for_update())
                    }
                }
                ted::replace(path.syntax(), res.syntax())
            }
            hir::PathResolution::Local(_)
            | hir::PathResolution::ConstParam(_)
            | hir::PathResolution::SelfType(_)
            | hir::PathResolution::Def(_)
            | hir::PathResolution::BuiltinAttr(_)
            | hir::PathResolution::ToolModule(_)
            | hir::PathResolution::DeriveHelper(_) => (),
        }
        Some(())
    }
}

// FIXME: It would probably be nicer if we could get this via HIR (i.e. get the
// trait ref, and then go from the types in the substs back to the syntax).
fn get_syntactic_substs(impl_def: ast::Impl) -> Option<Vec<ast::Type>> {
    let target_trait = impl_def.trait_()?;
    let path_type = match target_trait {
        ast::Type::PathType(path) => path,
        _ => return None,
    };
    let generic_arg_list = path_type.path()?.segment()?.generic_arg_list()?;

    get_type_args_from_arg_list(generic_arg_list)
}

fn get_type_args_from_arg_list(generic_arg_list: ast::GenericArgList) -> Option<Vec<ast::Type>> {
    let mut result = Vec::new();
    for generic_arg in generic_arg_list.generic_args() {
        if let ast::GenericArg::TypeArg(type_arg) = generic_arg {
            result.push(type_arg.ty()?)
        }
    }

    Some(result)
}

fn find_trait_for_assoc_item(
    scope: &SemanticsScope<'_>,
    type_param: hir::TypeParam,
    assoc_item: ast::NameRef,
) -> Option<hir::Trait> {
    let db = scope.db;
    let trait_bounds = type_param.trait_bounds(db);

    let assoc_item_name = assoc_item.text();

    for trait_ in trait_bounds {
        let names = trait_.items(db).into_iter().filter_map(|item| match item {
            hir::AssocItem::TypeAlias(ta) => Some(ta.name(db)),
            hir::AssocItem::Const(cst) => cst.name(db),
            _ => None,
        });

        for name in names {
            if assoc_item_name.as_str() == name.as_text()?.as_str() {
                // It is fine to return the first match because in case of
                // multiple possibilities, the exact trait must be disambiguated
                // in the definition of trait being implemented, so this search
                // should not be needed.
                return Some(trait_);
            }
        }
    }

    None
}
