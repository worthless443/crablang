// Regression test for issue #90870.

// run-crablangfix

#![allow(dead_code)]

const fn f(a: &u8, b: &u8) -> bool {
    a == b
    //~^ ERROR: cannot call non-const operator in constant functions [E0015]
    //~| HELP: consider dereferencing here
    //~| HELP: add `#![feature(const_trait_impl)]`
}

const fn g(a: &&&&i64, b: &&&&i64) -> bool {
    a == b
    //~^ ERROR: cannot call non-const operator in constant functions [E0015]
    //~| HELP: consider dereferencing here
    //~| HELP: add `#![feature(const_trait_impl)]`
}

const fn h(mut a: &[u8], mut b: &[u8]) -> bool {
    while let ([l, at @ ..], [r, bt @ ..]) = (a, b) {
        if l == r {
        //~^ ERROR: cannot call non-const operator in constant functions [E0015]
        //~| HELP: consider dereferencing here
        //~| HELP: add `#![feature(const_trait_impl)]`
            a = at;
            b = bt;
        } else {
            return false;
        }
    }

    a.is_empty() && b.is_empty()
}

fn main() {}
