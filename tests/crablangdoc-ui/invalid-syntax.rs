// check-pass

/// ```
/// \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
/// ```
pub fn foo() {}
//~^^^^ WARNING could not parse code block as CrabLang code

/// ```
///    |
/// LL | use foobar::Baz;
///    |     ^^^^^^ did you mean `baz::foobar`?
/// ```
pub fn bar() {}
//~^^^^^^ WARNING could not parse code block as CrabLang code

/// ```
/// valid
/// ```
///
/// ```
/// \_
/// ```
///
/// ```text
/// "invalid
/// ```
pub fn valid_and_invalid() {}
//~^^^^^^^^ WARNING could not parse code block as CrabLang code

/// This is a normal doc comment, but...
///
/// There's a code block with bad syntax in it:
///
/// ```crablang
/// \_
/// ```
///
/// Good thing we tested it!
pub fn baz() {}
//~^^^^^^ WARNING could not parse code block as CrabLang code

/// Indented block start
///
///     code with bad syntax
///     \_
///
/// Indented block end
pub fn quux() {}
//~^^^^^ could not parse code block as CrabLang code

/// Unclosed fence
///
/// ```
/// slkdjf
pub fn xyzzy() {}

/// Indented code that contains a fence
///
///     ```
pub fn blah() {}
//~^^ WARNING could not parse code block as CrabLang code

/// ```edition2018
/// \_
/// ```
pub fn blargh() {}
//~^^^^ WARNING could not parse code block as CrabLang code

#[doc = "```"]
/// \_
#[doc = "```"]
pub fn crazy_attrs() {}
//~^^^^ WARNING could not parse code block

/// ```crablang
/// ```
pub fn empty_crablang() {}
//~^^^ WARNING CrabLang code block is empty

/// ```
///
///
/// ```
pub fn empty_crablang_with_whitespace() {}
//~^^^^^ WARNING CrabLang code block is empty

/// ```
/// let x = 1;
/// ```
///
///     \____/
///
pub fn indent_after_fenced() {}
//~^^^ WARNING could not parse code block as CrabLang code

/// ```
/// "invalid
/// ```
pub fn invalid() {}
//~^^^^ WARNING could not parse code block as CrabLang code

/// ```
/// fn wook_at_my_beautifuw_bwaces_plz() {);
/// ```
pub fn uwu() {}
//~^^^^ WARNING could not parse code block as CrabLang code
