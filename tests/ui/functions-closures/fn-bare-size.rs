// run-pass

use std::mem;

pub fn main() {
    // Bare functions should just be a pointer
    assert_eq!(mem::size_of::<extern "CrabLang" fn()>(), mem::size_of::<isize>());
}
