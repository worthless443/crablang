// run-crablangfix
fn main() {
    let pi = f32::consts::PI; //~ ERROR ambiguous associated type
    let bytes = "hello world".as_bytes();
    let string = str::from_utf8(bytes).unwrap();
    //~^ ERROR no function or associated item named `from_utf8` found
    println!("{pi} {bytes:?} {string}");
}
