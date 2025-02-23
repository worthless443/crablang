use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet_with_applicability;
use crablangc_errors::Applicability;
use crablangc_hir::Expr;
use crablangc_lint::LateContext;
use crablangc_middle::ty::{self, Ty};

use super::FN_TO_NUMERIC_CAST_ANY;

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, cast_expr: &Expr<'_>, cast_from: Ty<'_>, cast_to: Ty<'_>) {
    // We allow casts from any function type to any function type.
    match cast_to.kind() {
        ty::FnDef(..) | ty::FnPtr(..) => return,
        _ => { /* continue to checks */ },
    }

    match cast_from.kind() {
        ty::FnDef(..) | ty::FnPtr(_) => {
            let mut applicability = Applicability::MaybeIncorrect;
            let from_snippet = snippet_with_applicability(cx, cast_expr.span, "..", &mut applicability);

            span_lint_and_sugg(
                cx,
                FN_TO_NUMERIC_CAST_ANY,
                expr.span,
                &format!("casting function pointer `{from_snippet}` to `{cast_to}`"),
                "did you mean to invoke the function?",
                format!("{from_snippet}() as {cast_to}"),
                applicability,
            );
        },
        _ => {},
    }
}
