// run-crablangfix
#![allow(unused_variables)]
fn main() {
    struct U;

    // A tuple is a "non-reference pattern".
    // A `mut` binding pattern resets the binding mode to by-value.

    let mut p = (U, U);
    let (a, mut b) = &mut p;
    //~^ ERROR cannot move out of a mutable reference
}
