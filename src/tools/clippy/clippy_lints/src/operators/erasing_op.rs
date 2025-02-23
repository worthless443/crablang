use clippy_utils::consts::{constant_simple, Constant};
use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::same_type_and_consts;

use crablangc_hir::{BinOpKind, Expr};
use crablangc_lint::LateContext;
use crablangc_middle::ty::TypeckResults;

use super::ERASING_OP;

pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'_>,
    op: BinOpKind,
    left: &'tcx Expr<'_>,
    right: &'tcx Expr<'_>,
) {
    let tck = cx.typeck_results();
    match op {
        BinOpKind::Mul | BinOpKind::BitAnd => {
            check_op(cx, tck, left, right, e);
            check_op(cx, tck, right, left, e);
        },
        BinOpKind::Div => check_op(cx, tck, left, right, e),
        _ => (),
    }
}

fn different_types(tck: &TypeckResults<'_>, input: &Expr<'_>, output: &Expr<'_>) -> bool {
    let input_ty = tck.expr_ty(input).peel_refs();
    let output_ty = tck.expr_ty(output).peel_refs();
    !same_type_and_consts(input_ty, output_ty)
}

fn check_op<'tcx>(
    cx: &LateContext<'tcx>,
    tck: &TypeckResults<'tcx>,
    op: &Expr<'tcx>,
    other: &Expr<'tcx>,
    parent: &Expr<'tcx>,
) {
    if constant_simple(cx, tck, op) == Some(Constant::Int(0)) {
        if different_types(tck, other, parent) {
            return;
        }
        span_lint(
            cx,
            ERASING_OP,
            parent.span,
            "this operation will always return zero. This is likely not the intended outcome",
        );
    }
}
