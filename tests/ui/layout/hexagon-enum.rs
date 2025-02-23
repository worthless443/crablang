// compile-flags: --target hexagon-unknown-linux-musl
// needs-llvm-components: hexagon
//
// Verify that the hexagon targets implement the repr(C) for enums correctly.
//
// See #82100
#![feature(never_type, crablangc_attrs, no_core, lang_items)]
#![crate_type = "lib"]
#![no_core]

#[lang="sized"]
trait Sized {}

#[crablangc_layout(debug)]
#[repr(C)]
enum A { Apple } //~ ERROR: layout_of

#[crablangc_layout(debug)]
#[repr(C)]
enum B { Banana = 255, } //~ ERROR: layout_of

#[crablangc_layout(debug)]
#[repr(C)]
enum C { Chaenomeles = 256, } //~ ERROR: layout_of

#[crablangc_layout(debug)]
#[repr(C)]
enum P { Peach = 0x1000_0000isize, } //~ ERROR: layout_of

const TANGERINE: usize = 0x8100_0000; // hack to get negative numbers without negation operator!

#[crablangc_layout(debug)]
#[repr(C)]
enum T { Tangerine = TANGERINE as isize } //~ ERROR: layout_of
