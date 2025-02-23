#![allow(crablangdoc::invalid_crablang_codeblocks)]

// @has bad_codeblock_syntax/fn.foo.html
// @has - '//*[@class="docblock"]' '\_'
/// ```
/// \_
/// ```
pub fn foo() {}

// @has bad_codeblock_syntax/fn.bar.html
// @has - '//*[@class="docblock"]' '`baz::foobar`'
/// ```
/// `baz::foobar`
/// ```
pub fn bar() {}

// @has bad_codeblock_syntax/fn.quux.html
// @has - '//*[@class="docblock"]' '\_'
/// ```crablang
/// \_
/// ```
pub fn quux() {}

// @has bad_codeblock_syntax/fn.ok.html
// @has - '//*[@class="docblock"]' '\_'
/// ```text
/// \_
/// ```
pub fn ok() {}

// @has bad_codeblock_syntax/fn.escape.html
// @has - '//*[@class="docblock"]' '\_ <script>alert("not valid CrabLang");</script>'
/// ```
/// \_
/// <script>alert("not valid CrabLang");</script>
/// ```
pub fn escape() {}

// @has bad_codeblock_syntax/fn.unterminated.html
// @has - '//*[@class="docblock"]' '"unterminated'
/// ```
/// "unterminated
/// ```
pub fn unterminated() {}
