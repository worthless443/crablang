use clippy_utils::consts::{constant, Constant};
use clippy_utils::diagnostics::span_lint;
use clippy_utils::is_trait_method;
use crablangc_hir as hir;
use crablangc_lint::LateContext;
use crablangc_span::sym;

use super::ITERATOR_STEP_BY_ZERO;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &hir::Expr<'_>, arg: &'tcx hir::Expr<'_>) {
    if is_trait_method(cx, expr, sym::Iterator) {
        if let Some((Constant::Int(0), _)) = constant(cx, cx.typeck_results(), arg) {
            span_lint(
                cx,
                ITERATOR_STEP_BY_ZERO,
                expr.span,
                "`Iterator::step_by(0)` will panic at runtime",
            );
        }
    }
}
