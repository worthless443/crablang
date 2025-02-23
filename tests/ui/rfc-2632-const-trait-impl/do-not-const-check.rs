// check-pass
#![feature(const_trait_impl, crablangc_attrs)]

#[const_trait]
trait IntoIter {
    fn into_iter(self);
}

#[const_trait]
trait Hmm: Sized {
    #[crablangc_do_not_const_check]
    fn chain<U>(self, other: U) where U: IntoIter,
    {
        other.into_iter()
    }
}

fn main() {}
