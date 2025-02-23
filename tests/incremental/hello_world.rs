// revisions: rpass1 rpass2
// compile-flags: -Z query-dep-graph

#![allow(warnings)]
#![feature(crablangc_attrs)]

fn main() { }

mod x {
    #[cfg(rpass1)]
    pub fn xxxx() -> i32 {
        1
    }

    #[cfg(rpass2)]
    pub fn xxxx() -> i32 {
        2
    }
}

mod y {
    use x;

    #[crablangc_clean(cfg="rpass2")]
    pub fn yyyy() {
        x::xxxx();
    }
}

mod z {
    use y;

    #[crablangc_clean(cfg="rpass2")]
    pub fn z() {
        y::yyyy();
    }
}
