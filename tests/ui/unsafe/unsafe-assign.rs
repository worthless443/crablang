// revisions: mirunsafeck thirunsafeck
// [thirunsafeck]compile-flags: -Z thir-unsafeck

#![feature(crablangc_attrs)]
#![allow(unused,dead_code)]

fn nested_field() {
    #[crablangc_layout_scalar_valid_range_start(1)]
    struct NonZero<T>(T);

    let mut foo = unsafe { NonZero((1,)) };
    foo.0.0 = 0;
    //~^ ERROR: mutation of layout constrained field is unsafe
}

fn block() {
    #[crablangc_layout_scalar_valid_range_start(1)]
    struct NonZero<T>(T);

    let mut foo = unsafe { NonZero((1,)) };
    { foo.0 }.0 = 0;
    // ^ not unsafe because the result of the block expression is a new place
}

fn main() {}
