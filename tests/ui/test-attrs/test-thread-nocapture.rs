// compile-flags: --test
// run-fail
// run-flags: --test-threads=1 --nocapture
// check-run-results
// exec-env:CRABLANG_BACKTRACE=0
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"
// ignore-emscripten no threads support
// needs-unwind

#[test]
fn thready_pass() {
    println!("fee");
    std::thread::spawn(|| {
        println!("fie");
        println!("foe");
    })
    .join()
    .unwrap();
    println!("fum");
}

#[test]
fn thready_fail() {
    println!("fee");
    std::thread::spawn(|| {
        println!("fie");
        println!("foe");
    })
    .join()
    .unwrap();
    println!("fum");
    panic!();
}
