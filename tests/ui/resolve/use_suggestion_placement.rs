// run-crablangfix
#![allow(dead_code)]

macro_rules! y {
    () => {}
}

mod m {
    pub const A: i32 = 0;
}

mod foo {
    #[derive(Debug)]
    pub struct Foo;

    // test whether the use suggestion isn't
    // placed into the expansion of `#[derive(Debug)]
    type Bar = Path; //~ ERROR cannot find
}

fn main() {
    y!();
    let _ = A; //~ ERROR cannot find
    foo();
}

fn foo() {
    type Dict<K, V> = HashMap<K, V>; //~ ERROR cannot find
}
