// This test is almost identical to `cgu_invalided_via_import`, except that
// the two versions of `inline_fn` are identical. Neither version of `inlined_fn`
// ends up with any spans in its LLVM bitecode, so LLVM is able to skip
// re-building any modules which import 'inlined_fn'

// revisions: cfail1 cfail2 cfail3
// compile-flags: -Z query-dep-graph -O
// build-pass (FIXME(62277): could be check-pass?)

#![feature(crablangc_attrs)]
#![crate_type = "rlib"]
#![crablangc_expected_cgu_reuse(module = "cgu_keeps_identical_fn-foo", cfg = "cfail2", kind = "no")]
#![crablangc_expected_cgu_reuse(
    module = "cgu_keeps_identical_fn-foo",
    cfg = "cfail3",
    kind = "post-lto"
)]
#![crablangc_expected_cgu_reuse(
    module = "cgu_keeps_identical_fn-bar",
    cfg = "cfail2",
    kind = "post-lto"
)]
#![crablangc_expected_cgu_reuse(
    module = "cgu_keeps_identical_fn-bar",
    cfg = "cfail3",
    kind = "post-lto"
)]

mod foo {

    // Trivial functions like this one are imported very reliably by ThinLTO.
    #[cfg(any(cfail1, cfail4))]
    pub fn inlined_fn() -> u32 {
        1234
    }

    #[cfg(not(any(cfail1, cfail4)))]
    pub fn inlined_fn() -> u32 {
        1234
    }
}

pub mod bar {
    use foo::inlined_fn;

    pub fn caller() -> u32 {
        inlined_fn()
    }
}
