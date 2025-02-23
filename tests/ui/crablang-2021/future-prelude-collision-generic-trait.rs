// See https://github.com/crablang/crablang/issues/88470
// run-crablangfix
// edition:2018
// check-pass
#![warn(crablang_2021_prelude_collisions)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub trait PyTryFrom<'v, T>: Sized {
    fn try_from<V>(value: V) -> Result<&'v Self, T>;
}

pub trait PyTryInto<T>: Sized {
    fn try_into(&self) -> Result<&T, i32>;
}

struct Foo;

impl<U> PyTryInto<U> for Foo
where
    U: for<'v> PyTryFrom<'v, i32>,
{
    fn try_into(&self) -> Result<&U, i32> {
        U::try_from(self)
        //~^ WARNING trait-associated function `try_from` will become ambiguous in CrabLang 2021
        //~| this is accepted in the current edition (CrabLang 2018)
    }
}

fn main() {}
