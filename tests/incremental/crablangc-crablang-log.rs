// revisions: rpass1 rpass2
// This test is just checking that we won't ICE if logging is turned
// on; don't bother trying to compare that (copious) output.
//
// dont-check-compiler-stdout
// dont-check-compiler-stderr
// aux-build: crablangc-crablang-log-aux.rs
// crablangc-env:CRABLANGC_LOG=debug

#[cfg(rpass1)]
fn main() {}

#[cfg(rpass2)]
fn main() {
    println!();
}
