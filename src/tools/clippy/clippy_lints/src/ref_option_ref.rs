use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::last_path_segment;
use clippy_utils::source::snippet;
use if_chain::if_chain;
use crablangc_errors::Applicability;
use crablangc_hir::{GenericArg, GenericArgsParentheses, Mutability, Ty, TyKind};
use crablangc_lint::{LateContext, LateLintPass};
use crablangc_session::{declare_lint_pass, declare_tool_lint};
use crablangc_span::symbol::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usage of `&Option<&T>`.
    ///
    /// ### Why is this bad?
    /// Since `&` is Copy, it's useless to have a
    /// reference on `Option<&T>`.
    ///
    /// ### Known problems
    /// It may be irrelevant to use this lint on
    /// public API code as it will make a breaking change to apply it.
    ///
    /// ### Example
    /// ```crablang,ignore
    /// let x: &Option<&u32> = &Some(&0u32);
    /// ```
    /// Use instead:
    /// ```crablang,ignore
    /// let x: Option<&u32> = Some(&0u32);
    /// ```
    #[clippy::version = "1.49.0"]
    pub REF_OPTION_REF,
    pedantic,
    "use `Option<&T>` instead of `&Option<&T>`"
}

declare_lint_pass!(RefOptionRef => [REF_OPTION_REF]);

impl<'tcx> LateLintPass<'tcx> for RefOptionRef {
    fn check_ty(&mut self, cx: &LateContext<'tcx>, ty: &'tcx Ty<'tcx>) {
        if_chain! {
            if let TyKind::Ref(_, ref mut_ty) = ty.kind;
            if mut_ty.mutbl == Mutability::Not;
            if let TyKind::Path(ref qpath) = &mut_ty.ty.kind;
            let last = last_path_segment(qpath);
            if let Some(def_id) = last.res.opt_def_id();

            if cx.tcx.is_diagnostic_item(sym::Option, def_id);
            if let Some(params) = last_path_segment(qpath).args ;
            if params.parenthesized == GenericArgsParentheses::No;
            if let Some(inner_ty) = params.args.iter().find_map(|arg| match arg {
                GenericArg::Type(inner_ty) => Some(inner_ty),
                _ => None,
            });
            if let TyKind::Ref(_, ref inner_mut_ty) = inner_ty.kind;
            if inner_mut_ty.mutbl == Mutability::Not;

            then {
                span_lint_and_sugg(
                    cx,
                    REF_OPTION_REF,
                    ty.span,
                    "since `&` implements the `Copy` trait, `&Option<&T>` can be simplified to `Option<&T>`",
                    "try",
                    format!("Option<{}>", &snippet(cx, inner_ty.span, "..")),
                    Applicability::MaybeIncorrect,
                );
            }
        }
    }
}
