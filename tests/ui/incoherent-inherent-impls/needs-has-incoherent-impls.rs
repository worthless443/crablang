// aux-build:extern-crate.rs
#![feature(crablangc_attrs)]
extern crate extern_crate;

impl extern_crate::StructWithAttr {
    //~^ ERROR cannot define inherent `impl` for a type outside of the crate
    fn foo() {}
}
impl extern_crate::StructWithAttr {
    #[crablangc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::StructNoAttr {
    //~^ ERROR cannot define inherent `impl` for a type outside of the crate
    fn foo() {}
}
impl extern_crate::StructNoAttr {
    //~^ ERROR cannot define inherent `impl` for a type outside of the crate
    #[crablangc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::EnumWithAttr {
    //~^ ERROR cannot define inherent `impl` for a type outside of the crate
    fn foo() {}
}
impl extern_crate::EnumWithAttr {
    #[crablangc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::EnumNoAttr {
    //~^ ERROR cannot define inherent `impl` for a type outside of the crate
    fn foo() {}
}
impl extern_crate::EnumNoAttr {
    //~^ ERROR cannot define inherent `impl` for a type outside of the crate
    #[crablangc_allow_incoherent_impl]
    fn bar() {}
}

fn main() {}
