// run-crablangfix
#![deny(crablang_2021_incompatible_closure_captures)]
//~^ NOTE: the lint level is defined here

// Test the two possible cases for automated migartion using crablangfix
// - Closure contains a block i.e.  `|| { .. };`
// - Closure contains just an expr `|| ..;`

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

fn closure_contains_block() {
    let t = (Foo(0), Foo(0));
    let c = || {
        //~^ ERROR: drop order
        //~| NOTE: for more information, see
        //~| HELP: add a dummy let to cause `t` to be fully captured
        let _t = t.0;
        //~^ NOTE: in CrabLang 2018, this closure captures all of `t`, but in CrabLang 2021, it will only capture `t.0`
    };

    c();
}
//~^ NOTE: in CrabLang 2018, `t` is dropped here, but in CrabLang 2021, only `t.0` will be dropped here as part of the closure

fn closure_doesnt_contain_block() {
    let t = (Foo(0), Foo(0));
    let c = || t.0;
    //~^ ERROR: drop order
    //~| NOTE: in CrabLang 2018, this closure captures all of `t`, but in CrabLang 2021, it will only capture `t.0`
    //~| NOTE: for more information, see
    //~| HELP: add a dummy let to cause `t` to be fully captured

    c();
}
//~^ NOTE: in CrabLang 2018, `t` is dropped here, but in CrabLang 2021, only `t.0` will be dropped here as part of the closure

fn main() {
    closure_contains_block();
    closure_doesnt_contain_block();
}
