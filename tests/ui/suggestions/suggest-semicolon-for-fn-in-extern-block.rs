// run-crablangfix

#[allow(dead_code)]

extern "C" {
  fn foo() //~ERROR expected `;`
}

fn main() {}
