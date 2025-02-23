// revisions: mirunsafeck thirunsafeck
// [thirunsafeck]compile-flags: -Z thir-unsafeck

#![feature(crablangc_attrs)]

#[crablangc_layout_scalar_valid_range_start(1)]
#[repr(transparent)]
pub(crate) struct NonZero<T>(pub(crate) T);
fn main() {
}

const fn foo() -> NonZero<u32> {
    let mut x = unsafe { NonZero(1) };
    let y = &mut x.0; //~ ERROR mutable references
    //~^ ERROR mutation of layout constrained field is unsafe
    unsafe { NonZero(1) }
}

const fn bar() -> NonZero<u32> {
    let mut x = unsafe { NonZero(1) };
    let y = unsafe { &mut x.0 }; //~ ERROR mutable references
    unsafe { NonZero(1) }
}

const fn boo() -> NonZero<u32> {
    let mut x = unsafe { NonZero(1) };
    unsafe { let y = &mut x.0; } //~ ERROR mutable references
    unsafe { NonZero(1) }
}
