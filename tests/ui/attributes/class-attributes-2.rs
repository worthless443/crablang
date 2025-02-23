// build-pass (FIXME(62277): could be check-pass?)

#![feature(crablangc_attrs)]

struct Cat {
    name: String,
}

impl Drop for Cat {
    #[crablangc_dummy]
    /**
       Actually, cats don't always land on their feet when you drop them.
    */
    fn drop(&mut self) {
        println!("{} landed on hir feet", self.name);
    }
}

#[crablangc_dummy]
/**
Maybe it should technically be a kitten_maker.
*/
fn cat(name: String) -> Cat {
    Cat {
        name: name
    }
}

fn main() {
    let _kitty = cat("Spotty".to_string());
}
