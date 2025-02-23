// run-crablangfix
#[derive(Debug, Default, Eq, PartialEq)]
struct A {
    b: u32,
    c: u64,
    d: usize,
}

fn a() {
    let q = A { c: 5..Default::default() };
    //~^ ERROR missing fields
    //~| HELP separate the last named field with a comma
    let r = A { c: 5, ..Default::default() };
    assert_eq!(q, r);
}

#[derive(Debug, Default, Eq, PartialEq)]
struct B {
    b: u32,
}

fn b() {
    let q = B { b: 1..Default::default() };
    //~^ ERROR mismatched types
    //~| HELP separate the last named field with a comma
    let r = B { b: 1 };
    assert_eq!(q, r);
}

fn main() {
    a();
    b();
}
