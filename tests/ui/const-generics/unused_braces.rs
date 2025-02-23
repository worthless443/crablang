// check-pass
// run-crablangfix
#![warn(unused_braces)]

macro_rules! make_1 {
    () => {
        1
    }
}

struct A<const N: usize>;

fn main() {
    let _: A<7>; // ok
    let _: A<{ 7 }>; //~ WARN unnecessary braces
    let _: A<{ 3 + 5 }>; // ok
    let _: A<{make_1!()}>; // ok
}
