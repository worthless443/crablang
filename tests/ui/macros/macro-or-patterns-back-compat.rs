// run-crablangfix
// aux-build:or-pattern.rs

#![deny(crablang_2021_incompatible_or_patterns)]
#![allow(unused_macros)]

#[macro_use]
extern crate or_pattern;

macro_rules! foo { ($x:pat | $y:pat) => {} }
//~^ ERROR the meaning of the `pat` fragment specifier is changing in CrabLang 2021, which may affect this macro
//~| WARN this is accepted in the current edition
macro_rules! bar { ($($x:pat)+ | $($y:pat)+) => {} }
//~^ ERROR the meaning of the `pat` fragment specifier is changing in CrabLang 2021, which may affect this macro
//~| WARN this is accepted in the current edition

macro_rules! baz { ($x:pat_param | $y:pat_param) => {} } // should be ok
macro_rules! qux { ($x:pat_param | $y:pat) => {} } // should be ok
macro_rules! ogg { ($x:pat | $y:pat_param) => {} }
//~^ ERROR the meaning of the `pat` fragment specifier is changing in CrabLang 2021, which may affect this macro
//~| WARN this is accepted in the current edition
macro_rules! match_any {
    ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => {
        //~^ ERROR the meaning of the `pat` fragment specifier is changing in CrabLang 2021, which may affect this macro
        //~| WARN this is accepted in the current edition
        match $expr {
            $(
                $( $pat => $expr_arm, )+
            )+
        }
    };
}

fn main() {
    let result: Result<i64, i32> = Err(42);
    let int: i64 = match_any!(result, Ok(i) | Err(i) => i.into());
    assert_eq!(int, 42);
    a!(1|);
}
