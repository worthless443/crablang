// run-crablangfix

#![allow(warnings)]

fn no_restriction<T>(x: &()) -> &() {
    with_restriction::<T>(x) //~ ERROR E0311
}

fn with_restriction<'a, T: 'a>(x: &'a ()) -> &'a () {
    x
}

fn main() {}
