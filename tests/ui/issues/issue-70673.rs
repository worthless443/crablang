// Regression test for https://github.com/crablang/crablang/issues/70673.

// run-pass

#![feature(thread_local)]

#[thread_local]
static A: &u8 = &42;

fn main() {
    dbg!(*A);
}
