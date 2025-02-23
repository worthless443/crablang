#![warn(anonymous_parameters)]
// Test for the anonymous_parameters deprecation lint (RFC 1685)

// check-pass
// edition:2015
// run-crablangfix

trait T {
    fn foo(i32); //~ WARNING anonymous parameters are deprecated
                 //~| WARNING this is accepted in the current edition

    fn bar_with_default_impl(String, String) {}
    //~^ WARNING anonymous parameters are deprecated
    //~| WARNING this is accepted in the current edition
    //~| WARNING anonymous parameters are deprecated
    //~| WARNING this is accepted in the current edition
}

fn main() {}
