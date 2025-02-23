// run-crablangfix
// edition:2018
// check-pass
#![warn(crablang_2021_prelude_collisions)]
#![allow(unreachable_code)]

macro_rules! foo {
    () => {{
        123;
        S
    }};
}

trait MyTry<T> {
    fn try_into(self, _: u8);
}

struct S;

impl MyTry<i32> for S {
    fn try_into(self, _: u8) {}
}

trait TryFromU8: Sized {
    fn try_from(_: u8);
}

impl TryFromU8 for u32 {
    fn try_from(_: u8) {}
}

macro_rules! bar {
    () => {
        u32
    };
}

fn main() {
    foo!().try_into(todo!());
    //~^ WARNING trait method `try_into` will become ambiguous in CrabLang 2021
    //~| WARNING this is accepted in the current edition
    <bar!()>::try_from(0);
    //~^ WARNING trait-associated function `try_from` will become ambiguous in CrabLang 2021
    //~| WARNING this is accepted in the current edition
}
