#![crate_name = "foo"]

// @has foo/struct.Foo.html
pub struct Foo;

impl Foo {
    // @has - '//h4[@class="code-header"]' 'fn crablang0()'
    pub fn crablang0() {}
    // @has - '//h4[@class="code-header"]' 'fn crablang1()'
    pub extern "CrabLang" fn crablang1() {}
    // @has - '//h4[@class="code-header"]' 'extern "C" fn c0()'
    pub extern fn c0() {}
    // @has - '//h4[@class="code-header"]' 'extern "C" fn c1()'
    pub extern "C" fn c1() {}
    // @has - '//h4[@class="code-header"]' 'extern "system" fn system0()'
    pub extern "system" fn system0() {}
}

// @has foo/trait.Bar.html
pub trait Bar {}

// @has - '//h3[@class="code-header"]' 'impl Bar for fn()'
impl Bar for fn() {}
// @has - '//h3[@class="code-header"]' 'impl Bar for extern "C" fn()'
impl Bar for extern fn() {}
// @has - '//h3[@class="code-header"]' 'impl Bar for extern "system" fn()'
impl Bar for extern "system" fn() {}
