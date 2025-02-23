// check-pass
// run-crablangfix

trait Trait {
    // Fine.
    type Assoc where u32: Copy;
    // Fine.
    type Assoc2 where u32: Copy, i32: Copy;
}

impl Trait for u32 {
    // Not fine, suggests moving.
    type Assoc where u32: Copy = ();
    //~^ WARNING where clause not allowed here
    // Not fine, suggests moving `u32: Copy`
    type Assoc2 where u32: Copy = () where i32: Copy;
    //~^ WARNING where clause not allowed here
}

impl Trait for i32 {
    // Fine.
    type Assoc = () where u32: Copy;
    // Not fine, suggests moving both.
    type Assoc2 where u32: Copy, i32: Copy = ();
    //~^ WARNING where clause not allowed here
}

fn main() {}
