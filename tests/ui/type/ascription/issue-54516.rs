// run-crablangfix
use std::collections::BTreeMap;

fn main() {
    println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
    //~^ ERROR type ascription cannot be followed by a function call
}
