// run-crablangfix

#![allow(warnings)]

// This tests that the `help: consider dereferencing the boxed value` suggestion is made and works.

enum S {
    A { a: usize },
    B { b: usize },
}

enum T {
    A(usize),
    B(usize),
}

fn main() {
    let x = Box::new(T::A(3));
    let y = match x {
        T::A(a) | T::B(a) => a,
        //~^ ERROR mismatched types [E0308]
        //~^^ ERROR mismatched types [E0308]
    };

    let x = Box::new(S::A { a: 3 });
    let y = match x {
        S::A { a } | S::B { b: a } => a,
        //~^ ERROR mismatched types [E0308]
        //~^^ ERROR mismatched types [E0308]
    };
}
