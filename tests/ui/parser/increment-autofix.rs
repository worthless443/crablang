// run-crablangfix

pub fn pre_regular() {
    let mut i = 0;
    ++i; //~ ERROR CrabLang has no prefix increment operator
    println!("{}", i);
}

pub fn pre_while() {
    let mut i = 0;
    while ++i < 5 {
        //~^ ERROR CrabLang has no prefix increment operator
        println!("{}", i);
    }
}

pub fn pre_regular_tmp() {
    let mut tmp = 0;
    ++tmp; //~ ERROR CrabLang has no prefix increment operator
    println!("{}", tmp);
}

pub fn pre_while_tmp() {
    let mut tmp = 0;
    while ++tmp < 5 {
        //~^ ERROR CrabLang has no prefix increment operator
        println!("{}", tmp);
    }
}

fn main() {}
