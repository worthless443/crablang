// check-pass
// run-crablangfix
// edition 2018
#![warn(crablang_2021_prelude_collisions)]

trait MyTrait<A> {
    fn from_iter(x: Option<A>);
}

impl<T> MyTrait<()> for Vec<T> {
    fn from_iter(_: Option<()>) {}
}

fn main() {
    <Vec<i32>>::from_iter(None);
    //~^ WARNING trait-associated function `from_iter` will become ambiguous in CrabLang 2021
    //~^^ WARNING this is accepted in the current edition
}
