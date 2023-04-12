// run-crablangfix

#![allow(dead_code)]

// This test checks that generic parameter re-ordering diagnostic suggestions contain bounds.

struct A;

impl A {
    pub fn do_things<T, 'a, 'b: 'a>() {
    //~^ ERROR lifetime parameters must be declared prior to type and const parameters
        println!("panic");
    }
}

fn main() {}
