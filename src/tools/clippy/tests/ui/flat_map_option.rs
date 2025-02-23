// run-crablangfix
#![warn(clippy::flat_map_option)]
#![allow(clippy::redundant_closure, clippy::unnecessary_filter_map)]

fn main() {
    // yay
    let c = |x| Some(x);
    let _ = [1].iter().flat_map(c);
    let _ = [1].iter().flat_map(Some);

    // nay
    let _ = [1].iter().flat_map(|_| &Some(1));
}
