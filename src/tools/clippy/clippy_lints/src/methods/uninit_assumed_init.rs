use clippy_utils::diagnostics::span_lint;
use clippy_utils::{is_path_diagnostic_item, ty::is_uninit_value_valid_for_ty};
use if_chain::if_chain;
use crablangc_hir as hir;
use crablangc_lint::LateContext;
use crablangc_span::sym;

use super::UNINIT_ASSUMED_INIT;

/// lint for `MaybeUninit::uninit().assume_init()` (we already have the latter)
pub(super) fn check(cx: &LateContext<'_>, expr: &hir::Expr<'_>, recv: &hir::Expr<'_>) {
    if_chain! {
        if let hir::ExprKind::Call(callee, args) = recv.kind;
        if args.is_empty();
        if is_path_diagnostic_item(cx, callee, sym::maybe_uninit_uninit);
        if !is_uninit_value_valid_for_ty(cx, cx.typeck_results().expr_ty_adjusted(expr));
        then {
            span_lint(
                cx,
                UNINIT_ASSUMED_INIT,
                expr.span,
                "this call for this type may be undefined behavior"
            );
        }
    }
}
