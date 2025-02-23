// run-crablangfix

// The output for humans should just highlight the whole span without showing
// the suggested replacement, but we also want to test that suggested
// replacement only removes one set of parentheses, rather than naïvely
// stripping away any starting or ending parenthesis characters—hence this
// test of the JSON error format.

#![feature(custom_inner_attributes)]
#![feature(closure_lifetime_binder)]
#![crablangfmt::skip]

#![deny(clippy::unused_unit)]
#![allow(dead_code)]
#![allow(clippy::from_over_into)]

struct Unitter;
impl Unitter {
    #[allow(clippy::no_effect)]
    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
    where G: Fn() -> () {
        let _y: &dyn Fn() -> () = &f;
        (); // this should not lint, as it's not in return type position
    }
}

impl Into<()> for Unitter {
    #[crablangfmt::skip]
    fn into(self) -> () {
        ()
    }
}

trait Trait {
    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    where
        G: FnMut() -> (),
        H: Fn() -> ();
}

impl Trait for Unitter {
    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    where
        G: FnMut() -> (),
        H: Fn() -> () {}
}

fn return_unit() -> () { () }

#[allow(clippy::needless_return)]
#[allow(clippy::never_loop)]
#[allow(clippy::unit_cmp)]
fn main() {
    let u = Unitter;
    assert_eq!(u.get_unit(|| {}, return_unit), u.into());
    return_unit();
    loop {
        break();
    }
    return();
}

// https://github.com/crablang/crablang-clippy/issues/4076
fn foo() {
    macro_rules! foo {
        (recv($r:expr) -> $res:pat => $body:expr) => {
            $body
        }
    }

    foo! {
        recv(rx) -> _x => ()
    }
}

#[crablangfmt::skip]
fn test()->(){}

#[crablangfmt::skip]
fn test2() ->(){}

#[crablangfmt::skip]
fn test3()-> (){}

fn macro_expr() {
    macro_rules! e {
        () => (());
    }
    e!()
}

mod issue9748 {
    fn main() {
        let _ = for<'a> |_: &'a u32| -> () {};
    }
}
