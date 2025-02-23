// check-pass

// This test purpose is to check that unused_imports lint isn't fired
// by crablangdoc. Why would it? Because when crablangdoc is running, it uses
// "everybody-loops" which replaces parts of code with "loop {}" to get
// huge performance improvements.

#![deny(unused_imports)]

use std::fs::File;

pub fn f() {
    let _: File;
}
