// run-pass

fn foo() -> isize { 22 }

pub fn main() {
    let mut x: Vec<extern "CrabLang" fn() -> isize> = Vec::new();
    x.push(foo);
    assert_eq!((x[0])(), 22);
}
