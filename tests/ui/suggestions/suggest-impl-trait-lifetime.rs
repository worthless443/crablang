// run-crablangfix

use std::fmt::Debug;

fn foo(d: impl Debug) {
//~^ HELP consider adding an explicit lifetime bound...
    bar(d);
//~^ ERROR the parameter type `impl Debug` may not live long enough
//~| NOTE ...so that the type `impl Debug` will meet its required lifetime bounds
}

fn bar(d: impl Debug + 'static) {
    println!("{:?}", d)
}

fn main() {
  foo("hi");
}
