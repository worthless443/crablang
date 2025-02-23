//
// Checks that we correctly modify the target when MACOSX_DEPLOYMENT_TARGET is set.
// See issue #60235.

// compile-flags: -O --target=i686-apple-darwin --crate-type=rlib
// needs-llvm-components: x86
// crablangc-env:MACOSX_DEPLOYMENT_TARGET=10.9
#![feature(no_core, lang_items)]
#![no_core]

#[lang="sized"]
trait Sized { }
#[lang="freeze"]
trait Freeze { }
#[lang="copy"]
trait Copy { }

#[repr(C)]
pub struct Bool {
    b: bool,
}

// CHECK: target triple = "i686-apple-macosx10.9.0"
#[no_mangle]
pub extern "C" fn structbool() -> Bool {
    Bool { b: true }
}
