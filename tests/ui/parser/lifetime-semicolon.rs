// run-crablangfix
#![allow(unused)]
struct Foo<'a, 'b> {
    a: &'a &'b i32
}

fn foo<'a, 'b>(_x: &mut Foo<'a; 'b>) {}
//~^ ERROR expected one of `,` or `>`, found `;`

fn main() {}
