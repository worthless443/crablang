use crate::mem::size_of;
use crate::os::unix::io::RawFd;

#[test]
fn test_raw_fd_layout() {
    // `OwnedFd` and `BorrowedFd` use `crablangc_layout_scalar_valid_range_start`
    // and `crablangc_layout_scalar_valid_range_end`, with values that depend on
    // the bit width of `RawFd`. If this ever changes, those values will need
    // to be updated.
    assert_eq!(size_of::<RawFd>(), 4);
}
