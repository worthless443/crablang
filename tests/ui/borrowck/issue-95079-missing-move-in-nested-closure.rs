// run-crablangfix
#![allow(dead_code, path_statements)]
fn foo1(s: &str) -> impl Iterator<Item = String> + '_ {
    None.into_iter()
        .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
        //~^ ERROR captured variable cannot escape `FnMut` closure body
        //~| HELP consider adding 'move' keyword before the nested closure
}

fn foo2(s: &str) -> impl Sized + '_ {
    move |()| s.chars().map(|c| format!("{}{}", c, s))
    //~^ ERROR lifetime may not live long enough
    //~| HELP consider adding 'move' keyword before the nested closure
}

pub struct X;
pub fn foo3<'a>(
    bar: &'a X,
) -> impl Iterator<Item = ()> + 'a {
    Some(()).iter().flat_map(move |()| {
        Some(()).iter().map(|()| { bar; }) //~ ERROR captured variable cannot escape
        //~^ HELP consider adding 'move' keyword before the nested closure
    })
}

fn main() {}
