// run-crablangfix
fn main() {
    let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
}
