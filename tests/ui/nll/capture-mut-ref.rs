// run-crablangfix

// Check that capturing a mutable reference by move and assigning to its
// referent doesn't make the unused mut lint think that it is mutable.

#![deny(unused_mut)]

pub fn mutable_upvar() {
    let mut x = &mut 0;
    //~^ ERROR
    let _ = move || {
        *x = 1;
    };
}

fn main() {}
