// revisions: edition2018 edition2021
//[edition2018] edition:2018
//[edition2021] edition:2021
// run-crablangfix

#![warn(clippy::uninlined_format_args)]

fn main() {
    let var = 1;

    println!("val='{}'", var);

    if var > 0 {
        panic!("p1 {}", var);
    }
    if var > 0 {
        panic!("p2 {0}", var);
    }
    if var > 0 {
        panic!("p3 {var}", var = var);
    }

    #[allow(non_fmt_panics)]
    {
        if var > 0 {
            panic!("p4 {var}");
        }
    }

    assert!(var == 1, "p5 {}", var);
    debug_assert!(var == 1, "p6 {}", var);
}
