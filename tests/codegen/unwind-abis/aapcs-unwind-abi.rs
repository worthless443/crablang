// needs-llvm-components: arm
// compile-flags: --target=armv7-unknown-linux-gnueabihf --crate-type=rlib -Cno-prepopulate-passes
#![no_core]
#![feature(no_core, lang_items, c_unwind)]
#[lang="sized"]
trait Sized { }

// Test that `nounwind` attributes are correctly applied to exported `aapcs` and
// `aapcs-unwind` extern functions. `aapcs-unwind` functions MUST NOT have this attribute. We
// disable optimizations above to prevent LLVM from inferring the attribute.

// CHECK: @crablang_item_that_cannot_unwind() unnamed_addr #0 {
#[no_mangle]
pub extern "aapcs" fn crablang_item_that_cannot_unwind() {
}

// CHECK: @crablang_item_that_can_unwind() unnamed_addr #1 {
#[no_mangle]
pub extern "aapcs-unwind" fn crablang_item_that_can_unwind() {
}

// Now, make some assertions that the LLVM attributes for these functions are correct.  First, make
// sure that the first item is correctly marked with the `nounwind` attribute:
//
// CHECK: attributes #0 = { {{.*}}nounwind{{.*}} }
//
// Next, let's assert that the second item, which CAN unwind, does not have this attribute.
//
// CHECK: attributes #1 = {
// CHECK-NOT: nounwind
// CHECK: }
