// run-crablangfix
trait Trait<A> {}

trait Assoc {
    type Ty;
}

impl<A> Assoc for dyn Trait<A> {
    type Ty = i32;
}

fn main() {
    let _x: <dyn Trait<i32>>::Ty; //~ ERROR ambiguous associated type
}
