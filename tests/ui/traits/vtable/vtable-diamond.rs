// build-fail
#![feature(crablangc_attrs)]

#[crablangc_dump_vtable]
trait A {
    fn foo_a(&self) {}
}

#[crablangc_dump_vtable]
trait B: A {
    fn foo_b(&self) {}
}

#[crablangc_dump_vtable]
trait C: A {
    //~^ error vtable
    fn foo_c(&self) {}
}

#[crablangc_dump_vtable]
trait D: B + C {
    //~^ error vtable
    fn foo_d(&self) {}
}

struct S;

impl A for S {}
impl B for S {}
impl C for S {}
impl D for S {}

fn foo(d: &dyn D) {
    d.foo_d();
}

fn bar(d: &dyn C) {
    d.foo_c();
}

fn main() {
    foo(&S);
    bar(&S);
}
