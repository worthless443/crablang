// https://github.com/crablang/crablang/issues/49081

// revisions:rpass1 rpass2

pub static A: i32 = 42;
pub static B: &i32 = &A;

fn main() {}
