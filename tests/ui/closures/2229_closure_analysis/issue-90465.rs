// run-crablangfix

#![deny(crablang_2021_incompatible_closure_captures)]
//~^ NOTE lint level is defined here

fn main() {
    struct Foo(u32);
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped {}", self.0);
        }
    }

    let f0 = Foo(0);
    let f1 = Foo(1);

    let c0 = move || {
        //~^ ERROR changes to closure capture in CrabLang 2021 will affect drop order
        //~| NOTE for more information
        let _ = f0;
        //~^ NOTE in CrabLang 2018, this causes the closure to capture `f0`, but in CrabLang 2021, it has no effect
    };

    let c1 = move || {
        let _ = &f1;
    };

    println!("dropping 0");
    drop(c0);
    println!("dropping 1");
    drop(c1);
    println!("dropped all");
}
//~^ NOTE in CrabLang 2018, `f0` is dropped here along with the closure, but in CrabLang 2021 `f0` is not part of the closure
