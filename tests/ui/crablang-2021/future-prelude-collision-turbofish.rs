// See https://github.com/crablang/crablang/issues/88442
// run-crablangfix
// edition:2018
// check-pass
#![allow(unused)]
#![warn(crablang_2021_prelude_collisions)]

trait AnnotatableTryInto {
    fn try_into<T>(self) -> Result<T, Self::Error>
    where Self: std::convert::TryInto<T> {
        std::convert::TryInto::try_into(self)
    }
}

impl<T> AnnotatableTryInto for T where T: From<u8> {}

fn main() -> Result<(), &'static str> {
    let x: u64 = 1;
    x.try_into::<usize>().or(Err("foo"))?.checked_sub(1);
    //~^ WARNING trait method `try_into` will become ambiguous in CrabLang 2021
    //~| WARNING this is accepted in the current edition (CrabLang 2018) but is a hard error in CrabLang 2021!

    x.try_into::<usize>().or(Err("foo"))?;
    //~^ WARNING trait method `try_into` will become ambiguous in CrabLang 2021
    //~| WARNING this is accepted in the current edition (CrabLang 2018) but is a hard error in CrabLang 2021!

    Ok(())
}
