use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::is_trait_method;
use crablangc_errors::Applicability;
use crablangc_hir as hir;
use crablangc_lint::LateContext;
use crablangc_middle::ty;
use crablangc_span::{source_map::Span, sym};

use super::FLAT_MAP_OPTION;
use clippy_utils::ty::is_type_diagnostic_item;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'_>, arg: &'tcx hir::Expr<'_>, span: Span) {
    if !is_trait_method(cx, expr, sym::Iterator) {
        return;
    }
    let arg_ty = cx.typeck_results().expr_ty_adjusted(arg);
    let sig = match arg_ty.kind() {
        ty::Closure(_, substs) => substs.as_closure().sig(),
        _ if arg_ty.is_fn() => arg_ty.fn_sig(cx.tcx),
        _ => return,
    };
    if !is_type_diagnostic_item(cx, sig.output().skip_binder(), sym::Option) {
        return;
    }
    span_lint_and_sugg(
        cx,
        FLAT_MAP_OPTION,
        span,
        "used `flat_map` where `filter_map` could be used instead",
        "try",
        "filter_map".into(),
        Applicability::MachineApplicable,
    );
}
