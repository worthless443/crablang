// run-crablangfix

#![deny(clippy::internal)]
#![allow(clippy::missing_clippy_version_attribute)]
#![feature(crablangc_private)]

extern crate crablangc_ast;
extern crate crablangc_hir;
extern crate crablangc_lint;
extern crate crablangc_middle;
#[macro_use]
extern crate crablangc_session;
use clippy_utils::extract_msrv_attr;
use clippy_utils::msrvs::Msrv;
use crablangc_hir::Expr;
use crablangc_lint::{EarlyContext, EarlyLintPass, LateContext, LateLintPass};

declare_lint! {
    pub TEST_LINT,
    Warn,
    ""
}

struct Pass {
    msrv: Msrv,
}

impl_lint_pass!(Pass => [TEST_LINT]);

impl LateLintPass<'_> for Pass {
    fn check_expr(&mut self, _: &LateContext<'_>, _: &Expr<'_>) {}
}

impl EarlyLintPass for Pass {
    fn check_expr(&mut self, _: &EarlyContext<'_>, _: &crablangc_ast::Expr) {}
}

fn main() {}
