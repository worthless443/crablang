// compile-flags:--crate-type lib
#![feature(decl_macro)]
#![feature(crablangc_attrs)]

#[crablangc_builtin_macro]
pub macro test($item:item) {
//~^ NOTE previously defined
    /* compiler built-in */
}

mod inner {
    #[crablangc_builtin_macro]
    pub macro test($item:item) {
    //~^ ERROR attempted to define built-in macro more than once [E0773]
        /* compiler built-in */
    }
}
