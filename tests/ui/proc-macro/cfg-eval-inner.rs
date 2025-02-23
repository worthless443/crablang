// compile-flags: -Z span-debug
// aux-build:test-macros.rs
// check-pass

#![feature(cfg_eval)]
#![feature(custom_inner_attributes)]
#![feature(stmt_expr_attributes)]
#![feature(crablangc_attrs)]

#![no_std] // Don't load unnecessary hygiene information from std
extern crate std;

#[macro_use]
extern crate test_macros;

struct Foo<T>(T);

impl Foo<[u8; {
    #![cfg_attr(not(FALSE), crablangc_dummy(cursed_inner))]
    #![allow(unused)]
    struct Inner {
        field: [u8; {
            #![cfg_attr(not(FALSE), crablangc_dummy(another_cursed_inner))]
            1
        }]
    }

    0
}]> {
    #![cfg_eval]
    #![print_attr]
    #![cfg_attr(not(FALSE), crablangc_dummy(evaluated_attr))]

    fn bar() {
        #[cfg(FALSE)] let a = 1;
    }
}

fn main() {}
