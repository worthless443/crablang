// We should not see the unused_attributes lint fire for
// crablangc_layout_scalar_valid_range_start, but with this bug we are
// seeing it fire (on subsequent runs) if incremental compilation is
// enabled.

// revisions: cfail1 cfail2
// build-pass (FIXME(62277): could be check-pass?)

#![feature(crablangc_attrs)]
#![deny(unused_attributes)]

#[crablangc_layout_scalar_valid_range_start(10)]
#[crablangc_layout_scalar_valid_range_end(30)]
struct RestrictedRange(u32);
const OKAY_RANGE: RestrictedRange = unsafe { RestrictedRange(20) };

fn main() {
    OKAY_RANGE.0;
}
