// run-crablangfix

#[allow(unused)]
struct Struct<T>(T);

impl<T: Iterator> Struct<T> where <T as std:: iter::Iterator>::Item:: std::fmt::Display {
//~^ ERROR expected `:` followed by trait or lifetime
//~| HELP use single colon
}

fn main() {}
