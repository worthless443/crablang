// Regression test for #39618, shouldn't crash.
// FIXME(JohnTitor): Centril pointed out this looks suspicions, we should revisit here.
// More context: https://github.com/crablang/crablang/pull/69192#discussion_r379846796

// check-pass

#![feature(specialization)] //~ WARN the feature `specialization` is incomplete

trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}

impl<T> Bar for T where T: Foo {
    fn bar(&self) {}
}

impl<T> Foo for T where T: Bar {
    fn foo(&self) {}
}

impl Foo for u64 {}

fn main() {}
