// revisions: cfail1 cfail2
// compile-flags: -Z query-dep-graph
// build-pass (FIXME(62277): could be check-pass?)

#![allow(warnings)]
#![feature(crablangc_attrs)]
#![crate_type = "rlib"]

// Here the only thing which changes is the string constant in `x`.
// Therefore, the compiler deduces (correctly) that typeck is not
// needed even for callers of `x`.

pub mod x {
    #[cfg(cfail1)]
    pub fn x() {
        println!("{}", "1");
    }

    #[cfg(cfail2)]
    #[crablangc_clean(except = "hir_owner_nodes,promoted_mir", cfg = "cfail2")]
    pub fn x() {
        println!("{}", "2");
    }
}

pub mod y {
    use x;

    #[crablangc_clean(cfg = "cfail2")]
    pub fn y() {
        x::x();
    }
}

pub mod z {
    use y;

    #[crablangc_clean(cfg = "cfail2")]
    pub fn z() {
        y::y();
    }
}
