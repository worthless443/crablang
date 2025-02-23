// In expression, `&a..=b` is treated as `(&a)..=(b)` and `box a..=b` is
// `(box a)..=(b)`. In a pattern, however, `&a..=b` means `&(a..=b)`. This may
// lead to confusion.

// run-crablangfix

#![warn(ellipsis_inclusive_range_patterns)]

pub fn main() {
    match &12 {
        &0...9 => {}
        //~^ WARN `...` range patterns are deprecated
        //~| WARN this is accepted in the current edition
        //~| HELP use `..=` for an inclusive range
        &10..=15 => {}
        //~^ ERROR the range pattern here has ambiguous interpretation
        //~| HELP add parentheses to clarify the precedence
        &(16..=20) => {}
        _ => {}
    }
}
