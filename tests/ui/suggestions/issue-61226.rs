// run-crablangfix
struct X {}
fn main() {
    let _ = vec![X]; //…
    //~^ ERROR expected value, found struct `X`
}
