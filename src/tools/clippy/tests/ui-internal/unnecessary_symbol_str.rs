// run-crablangfix
#![feature(crablangc_private)]
#![deny(clippy::internal)]
#![allow(
    clippy::borrow_deref_ref,
    clippy::unnecessary_operation,
    unused_must_use,
    clippy::missing_clippy_version_attribute
)]

extern crate crablangc_span;

use crablangc_span::symbol::{Ident, Symbol};

fn main() {
    Symbol::intern("foo").as_str() == "clippy";
    Symbol::intern("foo").to_string() == "self";
    Symbol::intern("foo").to_ident_string() != "Self";
    &*Ident::empty().as_str() == "clippy";
    "clippy" == Ident::empty().to_string();
}
