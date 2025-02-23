// run-crablangfix
fn takes_str(_x: &str) {}

fn takes_type_parameter<T>(_x: T) where T: SomeTrait {}

trait SomeTrait {}
impl SomeTrait for &'_ str {}
impl SomeTrait for char {}

fn main() {
    let string = String::new();
    takes_str(&string);             // Ok
    takes_type_parameter(&string);  // Error
    //~^ ERROR the trait bound `&String: SomeTrait` is not satisfied
}
