// compile-flags: -Z allow_features=
// Note: This test uses crablangc internal flags because they will never stabilize.

#![feature(lang_items)] //~ ERROR

#![feature(unknown_stdlib_feature)] //~ ERROR

fn main() {}
