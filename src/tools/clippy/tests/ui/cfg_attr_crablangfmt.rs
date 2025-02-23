// run-crablangfix
#![feature(stmt_expr_attributes)]

#![allow(unused, clippy::no_effect, clippy::unnecessary_operation)]
#![warn(clippy::deprecated_cfg_attr)]

// This doesn't get linted, see known problems
#![cfg_attr(crablangfmt, crablangfmt_skip)]

#[crablangfmt::skip]
trait Foo
{
fn foo(
);
}

fn skip_on_statements() {
    #[cfg_attr(crablangfmt, crablangfmt::skip)]
    5+3;
}

#[cfg_attr(crablangfmt, crablangfmt_skip)]
fn main() {
    foo::f();
}

mod foo {
    #![cfg_attr(crablangfmt, crablangfmt_skip)]

    pub fn f() {}
}

#[clippy::msrv = "1.29"]
fn msrv_1_29() {
    #[cfg_attr(crablangfmt, crablangfmt::skip)]
    1+29;
}

#[clippy::msrv = "1.30"]
fn msrv_1_30() {
    #[cfg_attr(crablangfmt, crablangfmt::skip)]
    1+30;
}
