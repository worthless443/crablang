// run-crablangfix

use std::ops::Index;
struct X;
impl Index<i32> for X {
    type Output = ();

    fn index(&self, _: i32) -> &() {
        &()
    }
}

fn main() {
    let x = vec![1, 2, 3];
    x[-1]; //~ ERROR negative integers cannot be used to index on a
    let x = [1, 2, 3];
    x[-1]; //~ ERROR negative integers cannot be used to index on a
    let x = &[1, 2, 3];
    x[-1]; //~ ERROR negative integers cannot be used to index on a
    let _ = x;
    X[-1];
}
