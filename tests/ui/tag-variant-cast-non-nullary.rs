// run-crablangfix
#![allow(dead_code, unused_variables)]
enum NonNullary {
    Nullary,
    Other(isize),
}

impl From<NonNullary> for isize {
    fn from(val: NonNullary) -> isize {
        match val {
            NonNullary::Nullary => 0,
            NonNullary::Other(i) => i,
        }
    }
}

fn main() {
    let v = NonNullary::Nullary;
    let val = v as isize; //~ ERROR non-primitive cast: `NonNullary` as `isize` [E0605]
}
