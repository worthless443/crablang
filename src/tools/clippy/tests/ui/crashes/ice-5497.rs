// reduced from crablangc issue-69020-assoc-const-arith-overflow.rs
pub fn main() {}

pub trait Foo {
    const OOB: i32;
}

impl<T: Foo> Foo for Vec<T> {
    const OOB: i32 = [1][1] + T::OOB;
    //~^ ERROR operation will panic
}
