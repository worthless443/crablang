use crate::{
    context::LintContext,
    lints::{EnumIntrinsicsMemDiscriminate, EnumIntrinsicsMemVariant},
    LateContext, LateLintPass,
};
use crablangc_hir as hir;
use crablangc_middle::ty::{visit::TypeVisitableExt, Ty};
use crablangc_span::{symbol::sym, Span};

declare_lint! {
    /// The `enum_intrinsics_non_enums` lint detects calls to
    /// intrinsic functions that require an enum ([`core::mem::discriminant`],
    /// [`core::mem::variant_count`]), but are called with a non-enum type.
    ///
    /// [`core::mem::discriminant`]: https://doc.crablang.org/core/mem/fn.discriminant.html
    /// [`core::mem::variant_count`]: https://doc.crablang.org/core/mem/fn.variant_count.html
    ///
    /// ### Example
    ///
    /// ```crablang,compile_fail
    /// #![deny(enum_intrinsics_non_enums)]
    /// core::mem::discriminant::<i32>(&123);
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// In order to accept any enum, the `mem::discriminant` and
    /// `mem::variant_count` functions are generic over a type `T`.
    /// This makes it technically possible for `T` to be a non-enum,
    /// in which case the return value is unspecified.
    ///
    /// This lint prevents such incorrect usage of these functions.
    ENUM_INTRINSICS_NON_ENUMS,
    Deny,
    "detects calls to `core::mem::discriminant` and `core::mem::variant_count` with non-enum types"
}

declare_lint_pass!(EnumIntrinsicsNonEnums => [ENUM_INTRINSICS_NON_ENUMS]);

/// Returns `true` if we know for sure that the given type is not an enum. Note that for cases where
/// the type is generic, we can't be certain if it will be an enum so we have to assume that it is.
fn is_non_enum(t: Ty<'_>) -> bool {
    !t.is_enum() && !t.needs_subst()
}

fn enforce_mem_discriminant(
    cx: &LateContext<'_>,
    func_expr: &hir::Expr<'_>,
    expr_span: Span,
    args_span: Span,
) {
    let ty_param = cx.typeck_results().node_substs(func_expr.hir_id).type_at(0);
    if is_non_enum(ty_param) {
        cx.emit_spanned_lint(
            ENUM_INTRINSICS_NON_ENUMS,
            expr_span,
            EnumIntrinsicsMemDiscriminate { ty_param, note: args_span },
        );
    }
}

fn enforce_mem_variant_count(cx: &LateContext<'_>, func_expr: &hir::Expr<'_>, span: Span) {
    let ty_param = cx.typeck_results().node_substs(func_expr.hir_id).type_at(0);
    if is_non_enum(ty_param) {
        cx.emit_spanned_lint(
            ENUM_INTRINSICS_NON_ENUMS,
            span,
            EnumIntrinsicsMemVariant { ty_param },
        );
    }
}

impl<'tcx> LateLintPass<'tcx> for EnumIntrinsicsNonEnums {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &hir::Expr<'_>) {
        let hir::ExprKind::Call(func, args) = &expr.kind else { return };
        let hir::ExprKind::Path(qpath) = &func.kind else { return };
        let Some(def_id) = cx.qpath_res(qpath, func.hir_id).opt_def_id() else { return };
        let Some(name) = cx.tcx.get_diagnostic_name(def_id) else { return };
        match name {
            sym::mem_discriminant => enforce_mem_discriminant(cx, func, expr.span, args[0].span),
            sym::mem_variant_count => enforce_mem_variant_count(cx, func, expr.span),
            _ => {}
        }
    }
}
