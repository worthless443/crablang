// run-crablangfix
// edition:2018
// check-pass
#![warn(crablang_2021_compatibility)]

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

macro_rules! m {
    (@ $body:expr) => {{
        let f = || $body;
        //~^ WARNING: drop order
        f();
    }};
    ($body:block) => {{
        m!(@ $body);
    }};
}

fn main() {
    let a = (Foo(0), Foo(1));
    m!({
        //~^ HELP: add a dummy
        let x = a.0;
        println!("{:?}", x);
    });
}
