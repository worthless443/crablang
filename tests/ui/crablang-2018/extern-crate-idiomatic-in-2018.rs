// aux-build:edition-lint-paths.rs
// run-crablangfix
// compile-flags:--extern edition_lint_paths
// edition:2018

// The "normal case". Ideally we would remove the `extern crate` here,
// but we don't.

#![deny(crablang_2018_idioms)]
#![allow(dead_code)]

extern crate edition_lint_paths;
//~^ ERROR unused extern crate

// Shouldn't suggest changing to `use`, as `bar`
// would no longer be added to the prelude which could cause
// compilation errors for imports that use `bar` in other
// modules. See #57672.
extern crate edition_lint_paths as bar;

fn main() {
    // This is not considered to *use* the `extern crate` in CrabLang 2018:
    use edition_lint_paths::foo;
    foo();

    // But this should be a use of the (renamed) crate:
    crate::bar::foo();
}
