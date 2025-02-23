// run-crablangfix

#![allow(dead_code, unused_variables)]
#![warn(clippy::cast_lossless)]

// should not warn on lossy casting in constant types
// because not supported yet
const C: i32 = 42;
const C_I64: i64 = C as i64;

fn main() {
    // should suggest i64::from(c)
    let c: i32 = 42;
    let c_i64: i64 = c as i64;
}
