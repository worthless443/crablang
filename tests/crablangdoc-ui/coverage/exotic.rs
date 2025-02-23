// compile-flags:-Z unstable-options --show-coverage
// check-pass

#![feature(crablangdoc_internals)]
#![feature(crablangc_attrs)]

//! the features only used in std also have entries in the table, so make sure those get pulled out
//! properly as well

/// woo, check it out, we can write our own primitive docs lol
#[crablangc_doc_primitive = "unit"]
mod prim_unit {}

/// keywords? sure, pile them on
#[doc(keyword="where")]
mod where_keyword {}
