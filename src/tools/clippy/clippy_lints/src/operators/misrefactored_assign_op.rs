use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet_opt;
use clippy_utils::sugg;
use crablangc_errors::Applicability;
use crablangc_hir as hir;
use crablangc_lint::LateContext;

use super::MISREFACTORED_ASSIGN_OP;

pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx hir::Expr<'_>,
    op: hir::BinOpKind,
    lhs: &'tcx hir::Expr<'_>,
    rhs: &'tcx hir::Expr<'_>,
) {
    if let hir::ExprKind::Binary(binop, l, r) = &rhs.kind {
        if op != binop.node {
            return;
        }
        // lhs op= l op r
        if eq_expr_value(cx, lhs, l) {
            lint_misrefactored_assign_op(cx, expr, op, rhs, lhs, r);
        }
        // lhs op= l commutative_op r
        if is_commutative(op) && eq_expr_value(cx, lhs, r) {
            lint_misrefactored_assign_op(cx, expr, op, rhs, lhs, l);
        }
    }
}

fn lint_misrefactored_assign_op(
    cx: &LateContext<'_>,
    expr: &hir::Expr<'_>,
    op: hir::BinOpKind,
    rhs: &hir::Expr<'_>,
    assignee: &hir::Expr<'_>,
    rhs_other: &hir::Expr<'_>,
) {
    span_lint_and_then(
        cx,
        MISREFACTORED_ASSIGN_OP,
        expr.span,
        "variable appears on both sides of an assignment operation",
        |diag| {
            if let (Some(snip_a), Some(snip_r)) = (snippet_opt(cx, assignee.span), snippet_opt(cx, rhs_other.span)) {
                let a = &sugg::Sugg::hir(cx, assignee, "..");
                let r = &sugg::Sugg::hir(cx, rhs, "..");
                let long = format!("{snip_a} = {}", sugg::make_binop(op.into(), a, r));
                diag.span_suggestion(
                    expr.span,
                    format!(
                        "did you mean `{snip_a} = {snip_a} {} {snip_r}` or `{long}`? Consider replacing it with",
                        op.as_str()
                    ),
                    format!("{snip_a} {}= {snip_r}", op.as_str()),
                    Applicability::MaybeIncorrect,
                );
                diag.span_suggestion(
                    expr.span,
                    "or",
                    long,
                    Applicability::MaybeIncorrect, // snippet
                );
            }
        },
    );
}

#[must_use]
fn is_commutative(op: hir::BinOpKind) -> bool {
    use crablangc_hir::BinOpKind::{
        Add, And, BitAnd, BitOr, BitXor, Div, Eq, Ge, Gt, Le, Lt, Mul, Ne, Or, Rem, Shl, Shr, Sub,
    };
    match op {
        Add | Mul | And | Or | BitXor | BitAnd | BitOr | Eq | Ne => true,
        Sub | Div | Rem | Shl | Shr | Lt | Le | Ge | Gt => false,
    }
}
