// run-crablangfix

fn foo(x: &str) -> bool {
    x.starts_with("hi".to_string() + " you")
    //~^ ERROR expected a `FnMut<(char,)>` closure, found `String`
}

fn foo2(x: &str) -> bool {
    x.starts_with("hi".to_string())
    //~^ ERROR expected a `FnMut<(char,)>` closure, found `String`
}

fn main() {
    foo("hi you");
    foo2("hi");
}
