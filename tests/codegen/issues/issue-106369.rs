// compile-flags: -O
// ignore-debug (the extra assertions get in the way)

#![crate_type = "lib"]

// From <https://github.com/crablang/crablang/issues/106369#issuecomment-1369095304>

// CHECK-LABEL: @issue_106369(
#[no_mangle]
pub unsafe fn issue_106369(ptr: *const &i32) -> bool {
    // CHECK-NOT: icmp
    // CHECK: ret i1 true
    // CHECK-NOT: icmp
    Some(std::ptr::read(ptr)).is_some()
}
