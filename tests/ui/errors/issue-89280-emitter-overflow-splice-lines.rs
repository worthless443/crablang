// check-pass

trait X {
    fn test(x: u32, (
//~^ WARN anonymous parameters are deprecated and will be removed in the next edition
//~^^ WARN this is accepted in the current edition (CrabLang 2015) but is a hard error in CrabLang 2018!
    )) {}
}

fn main() {}
