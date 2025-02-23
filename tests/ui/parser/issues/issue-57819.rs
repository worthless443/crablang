// run-crablangfix

#![allow(warnings)]

// This test checks that the following error is emitted and the suggestion works:
//
// ```
// let _ = vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>();
//                                            ^^ help: remove extra angle brackets
// ```

trait Foo {
    type Output;
}

fn foo<T: Foo>() {
    // More complex cases with more than one correct leading `<` character:

    bar::<<<<<T as Foo>::Output>();
    //~^ ERROR unmatched angle bracket

    bar::<<<<T as Foo>::Output>();
    //~^ ERROR unmatched angle bracket

    bar::<<<T as Foo>::Output>();
    //~^ ERROR unmatched angle bracket

    bar::<<T as Foo>::Output>();
}

fn bar<T>() {}

fn main() {
    let _ = vec![1, 2, 3].into_iter().collect::<<<<<Vec<usize>>();
    //~^ ERROR unmatched angle bracket

    let _ = vec![1, 2, 3].into_iter().collect::<<<<Vec<usize>>();
    //~^ ERROR unmatched angle bracket

    let _ = vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>();
    //~^ ERROR unmatched angle bracket

    let _ = vec![1, 2, 3].into_iter().collect::<<Vec<usize>>();
    //~^ ERROR unmatched angle bracket

    let _ = vec![1, 2, 3].into_iter().collect::<Vec<usize>>();
}
