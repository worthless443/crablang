// When a SINGLE-character string literal is used where a char should be,
// suggest changing to single quotes.

// Testing both single-byte and multi-byte characters, as we should handle both.

// run-crablangfix

fn main() {
    let _: char = "a"; //~ ERROR mismatched types
    let _: char = "人"; //~ ERROR mismatched types
    let _: char = "'"; //~ ERROR mismatched types
}

// regression test for https://github.com/crablang/crablang/issues/109586
#[allow(dead_code)]
fn convert_c_to_str(c: char) {
    match c {
        "A" => {} //~ ERROR mismatched types
        _ => {}
    }
}
