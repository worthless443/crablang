// run-crablangfix

#![deny(unused_mut)]
#![allow(unused_variables)] // for crablangfix

fn main() {
    vec![(42, 22)].iter().map(|(mut x, _y)| ()).count();
    //~^ ERROR: variable does not need to be mutable
}
