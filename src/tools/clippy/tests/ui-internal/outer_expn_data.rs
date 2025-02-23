// run-crablangfix

#![deny(clippy::internal)]
#![allow(clippy::missing_clippy_version_attribute)]
#![feature(crablangc_private)]

extern crate crablangc_hir;
extern crate crablangc_lint;
extern crate crablangc_middle;
#[macro_use]
extern crate crablangc_session;
use crablangc_hir::Expr;
use crablangc_lint::{LateContext, LateLintPass};

declare_lint! {
    pub TEST_LINT,
    Warn,
    ""
}

declare_lint_pass!(Pass => [TEST_LINT]);

impl<'tcx> LateLintPass<'tcx> for Pass {
    fn check_expr(&mut self, _cx: &LateContext<'tcx>, expr: &'tcx Expr) {
        let _ = expr.span.ctxt().outer_expn().expn_data();
    }
}

fn main() {}
