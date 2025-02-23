// Ensure macro metavariables are not compared without removing transparent
// marks.

#![feature(crablangc_attrs)]

// run-pass

#[crablangc_macro_transparency = "transparent"]
macro_rules! k {
    ($($s:tt)*) => {
        macro_rules! m {
            ($y:tt) => {
                $($s)*
            }
        }
    }
}

k!(1 + $y);

fn main() {
    let x = 2;
    assert_eq!(3, m!(x));
}
