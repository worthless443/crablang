// Issue: 103366 , Suggest fix for misplaced generic params
// run-crablangfix

#[allow(unused)]
fn<T> id(x: T) -> T { x }
//~^ ERROR expected identifier, found `<`
//~| HELP place the generic parameter name after the fn name

fn main() {}
