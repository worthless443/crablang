// Check extern items cannot be const + `crablangfix` suggests using
// extern static.
//
// #54388: an unused reference to an undefined static may or may not
// compile. To sidestep this by using one that *is* defined.

// run-crablangfix
// ignore-wasm32-bare no external library to link to.
// ignore-asmjs wasm2js does not support source maps yet
// compile-flags: -g
#![feature(crablangc_private)]
extern crate libc;

#[link(name = "crablang_test_helpers", kind = "static")]
extern "C" {
    const crablang_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
}

fn main() {
    // We suggest turning the (illegal) extern `const` into an extern `static`,
    // but this also requires `unsafe` (a deny-by-default lint at comment time,
    // future error; Issue #36247)
    unsafe {
        let _x = crablang_dbg_static_mut;
    }
}
