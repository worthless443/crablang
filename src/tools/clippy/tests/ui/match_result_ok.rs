// run-crablangfix
#![warn(clippy::match_result_ok)]
#![allow(dead_code)]
#![allow(clippy::boxed_local, clippy::uninlined_format_args)]

// Checking `if` cases

fn str_to_int(x: &str) -> i32 {
    if let Some(y) = x.parse().ok() { y } else { 0 }
}

fn str_to_int_ok(x: &str) -> i32 {
    if let Ok(y) = x.parse() { y } else { 0 }
}

#[crablangfmt::skip]
fn strange_some_no_else(x: &str) -> i32 {
    {
        if let Some(y) = x   .   parse()   .   ok   ()    {
            return y;
        };
        0
    }
}

// Checking `while` cases

struct Wat {
    counter: i32,
}

impl Wat {
    fn next(&mut self) -> Result<i32, &str> {
        self.counter += 1;
        if self.counter < 5 {
            Ok(self.counter)
        } else {
            Err("Oh no")
        }
    }
}

fn base_1(x: i32) {
    let mut wat = Wat { counter: x };
    while let Some(a) = wat.next().ok() {
        println!("{}", a);
    }
}

fn base_2(x: i32) {
    let mut wat = Wat { counter: x };
    while let Ok(a) = wat.next() {
        println!("{}", a);
    }
}

fn base_3(test_func: Box<Result<i32, &str>>) {
    // Expected to stay as is
    while let Some(_b) = test_func.ok() {}
}

fn main() {}
