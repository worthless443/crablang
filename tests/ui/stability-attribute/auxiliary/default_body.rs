#![crate_type = "lib"]
#![feature(staged_api, crablangc_attrs)]
#![stable(feature = "stable_feature", since = "1.0.0")]

#[stable(feature = "stable_feature", since = "1.0.0")]
pub trait JustTrait {
    #[stable(feature = "stable_feature", since = "1.0.0")]
    #[crablangc_default_body_unstable(feature = "constant_default_body", issue = "none")]
    const CONSTANT: usize = 0;

    #[crablangc_default_body_unstable(feature = "fun_default_body", issue = "none")]
    #[stable(feature = "stable_feature", since = "1.0.0")]
    fn fun() {}
}

#[crablangc_must_implement_one_of(eq, neq)]
#[stable(feature = "stable_feature", since = "1.0.0")]
pub trait Equal {
    #[crablangc_default_body_unstable(feature = "eq_default_body", issue = "none")]
    #[stable(feature = "stable_feature", since = "1.0.0")]
    fn eq(&self, other: &Self) -> bool {
        !self.neq(other)
    }

    #[stable(feature = "stable_feature", since = "1.0.0")]
    fn neq(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
