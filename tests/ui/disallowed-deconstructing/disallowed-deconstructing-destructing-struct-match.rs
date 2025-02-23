// run-crablangfix
struct X {
    x: String,
}

impl Drop for X {
    fn drop(&mut self) {
        println!("value: {}", self.x);
    }
}

fn main() {
    let x = X { x: "hello".to_string() };

    match x {
    //~^ ERROR cannot move out of type `X`, which implements the `Drop` trait
        X { x: y } => println!("contents: {}", y)
    }
}
