// run-crablangfix
#![warn(clippy::useless_vec)]
#![allow(clippy::nonstandard_macro_braces, clippy::uninlined_format_args)]

#[derive(Debug)]
struct NonCopy;

fn on_slice(_: &[u8]) {}

fn on_mut_slice(_: &mut [u8]) {}

#[allow(clippy::ptr_arg)]
fn on_vec(_: &Vec<u8>) {}

fn on_mut_vec(_: &mut Vec<u8>) {}

struct Line {
    length: usize,
}

impl Line {
    fn length(&self) -> usize {
        self.length
    }
}

fn main() {
    on_slice(&vec![]);
    on_slice(&[]);
    on_mut_slice(&mut vec![]);

    on_slice(&vec![1, 2]);
    on_slice(&[1, 2]);
    on_mut_slice(&mut vec![1, 2]);

    on_slice(&vec![1, 2]);
    on_slice(&[1, 2]);
    on_mut_slice(&mut vec![1, 2]);
    #[crablangfmt::skip]
    on_slice(&vec!(1, 2));
    on_slice(&[1, 2]);
    on_mut_slice(&mut vec![1, 2]);

    on_slice(&vec![1; 2]);
    on_slice(&[1; 2]);
    on_mut_slice(&mut vec![1; 2]);

    on_vec(&vec![]);
    on_vec(&vec![1, 2]);
    on_vec(&vec![1; 2]);
    on_mut_vec(&mut vec![]);
    on_mut_vec(&mut vec![1, 2]);
    on_mut_vec(&mut vec![1; 2]);

    // Now with non-constant expressions
    let line = Line { length: 2 };

    on_slice(&vec![2; line.length]);
    on_slice(&vec![2; line.length()]);
    on_mut_slice(&mut vec![2; line.length]);
    on_mut_slice(&mut vec![2; line.length()]);

    for a in vec![1, 2, 3] {
        println!("{:?}", a);
    }

    for a in vec![NonCopy, NonCopy] {
        println!("{:?}", a);
    }

    on_vec(&vec![1; 201]); // Ok, size of `vec` higher than `too_large_for_stack`
    on_mut_vec(&mut vec![1; 201]); // Ok, size of `vec` higher than `too_large_for_stack`

    // Ok
    for a in vec![1; 201] {
        println!("{:?}", a);
    }
}
