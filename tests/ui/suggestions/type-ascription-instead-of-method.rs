// run-crablangfix
fn main() {
    let _ = Box:new("foo".to_string());
    //~^ ERROR expected type, found
}
