// compile-flags: --test
// run-crablangfix
// Checks that the `use` suggestion appears *below* this inner attribute.
// There was an issue where the test synthetic #[allow(dead)] attribute on
// main which has a dummy span caused the suggestion to be placed at the top
// of the file.
#![allow(unused)]

fn main() {
    let s = m::S;
    s.abc(); //~ ERROR no method named `abc`
}

mod m {
    pub trait Foo {
        fn abc(&self) {}
    }
    pub struct S;
    impl Foo for S{}
}
