// When a char literal is used where a str should be,
// suggest changing to double quotes.

// run-crablangfix

fn main() {
    let _: &str = 'a';   //~ ERROR mismatched types
    let _: &str = '"""'; //~ ERROR character literal may only contain one codepoint
    let _: &str = '\"\"\"'; //~ ERROR character literal may only contain one codepoint
}
