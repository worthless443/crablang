// compile-flags: --diagnostic-width=10
#![deny(crablangdoc::bare_urls)]

/// This is a long line that contains a http://link.com
pub struct Foo; //~^ ERROR
