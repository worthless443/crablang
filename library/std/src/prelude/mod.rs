//! # The CrabLang Prelude
//!
//! CrabLang comes with a variety of things in its standard library. However, if
//! you had to manually import every single thing that you used, it would be
//! very verbose. But importing a lot of things that a program never uses isn't
//! good either. A balance needs to be struck.
//!
//! The *prelude* is the list of things that CrabLang automatically imports into
//! every CrabLang program. It's kept as small as possible, and is focused on
//! things, particularly traits, which are used in almost every single CrabLang
//! program.
//!
//! # Other preludes
//!
//! Preludes can be seen as a pattern to make using multiple types more
//! convenient. As such, you'll find other preludes in the standard library,
//! such as [`std::io::prelude`]. Various libraries in the CrabLang ecosystem may
//! also define their own preludes.
//!
//! [`std::io::prelude`]: crate::io::prelude
//!
//! The difference between 'the prelude' and these other preludes is that they
//! are not automatically `use`'d, and must be imported manually. This is still
//! easier than importing all of their constituent components.
//!
//! # Prelude contents
//!
//! The first version of the prelude is used in CrabLang 2015 and CrabLang 2018,
//! and lives in [`std::prelude::v1`].
//! [`std::prelude::crablang_2015`] and [`std::prelude::crablang_2018`] re-export this prelude.
//! It re-exports the following:
//!
//! * <code>[std::marker]::{[Copy], [Send], [Sized], [Sync], [Unpin]}</code>,
//!   marker traits that indicate fundamental properties of types.
//! * <code>[std::ops]::{[Drop], [Fn], [FnMut], [FnOnce]}</code>, various
//!   operations for both destructors and overloading `()`.
//! * <code>[std::mem]::[drop][mem::drop]</code>, a convenience function for explicitly
//!   dropping a value.
//! * <code>[std::boxed]::[Box]</code>, a way to allocate values on the heap.
//! * <code>[std::borrow]::[ToOwned]</code>, the conversion trait that defines
//!   [`to_owned`], the generic method for creating an owned type from a
//!   borrowed type.
//! * <code>[std::clone]::[Clone]</code>, the ubiquitous trait that defines
//!   [`clone`][Clone::clone], the method for producing a copy of a value.
//! * <code>[std::cmp]::{[PartialEq], [PartialOrd], [Eq], [Ord]}</code>, the
//!   comparison traits, which implement the comparison operators and are often
//!   seen in trait bounds.
//! * <code>[std::convert]::{[AsRef], [AsMut], [Into], [From]}</code>, generic
//!   conversions, used by savvy API authors to create overloaded methods.
//! * <code>[std::default]::[Default]</code>, types that have default values.
//! * <code>[std::iter]::{[Iterator], [Extend], [IntoIterator], [DoubleEndedIterator], [ExactSizeIterator]}</code>,
//!   iterators of various
//!   kinds.
//! * <code>[std::option]::[Option]::{[self][Option], [Some], [None]}</code>, a
//!   type which expresses the presence or absence of a value. This type is so
//!   commonly used, its variants are also exported.
//! * <code>[std::result]::[Result]::{[self][Result], [Ok], [Err]}</code>, a type
//!   for functions that may succeed or fail. Like [`Option`], its variants are
//!   exported as well.
//! * <code>[std::string]::{[String], [ToString]}</code>, heap-allocated strings.
//! * <code>[std::vec]::[Vec]</code>, a growable, heap-allocated vector.
//!
//! The prelude used in CrabLang 2021, [`std::prelude::crablang_2021`], includes all of the above,
//! and in addition re-exports:
//!
//! * <code>[std::convert]::{[TryFrom], [TryInto]}</code>,
//! * <code>[std::iter]::[FromIterator]</code>.
//!
//! [mem::drop]: crate::mem::drop
//! [std::borrow]: crate::borrow
//! [std::boxed]: crate::boxed
//! [std::clone]: crate::clone
//! [std::cmp]: crate::cmp
//! [std::convert]: crate::convert
//! [std::default]: crate::default
//! [std::iter]: crate::iter
//! [std::marker]: crate::marker
//! [std::mem]: crate::mem
//! [std::ops]: crate::ops
//! [std::option]: crate::option
//! [`std::prelude::v1`]: v1
//! [`std::prelude::crablang_2015`]: crablang_2015
//! [`std::prelude::crablang_2018`]: crablang_2018
//! [`std::prelude::crablang_2021`]: crablang_2021
//! [std::result]: crate::result
//! [std::slice]: crate::slice
//! [std::string]: crate::string
//! [std::vec]: mod@crate::vec
//! [TryFrom]: crate::convert::TryFrom
//! [TryInto]: crate::convert::TryInto
//! [FromIterator]: crate::iter::FromIterator
//! [`to_owned`]: crate::borrow::ToOwned::to_owned
//! [book-closures]: ../../book/ch13-01-closures.html
//! [book-dtor]: ../../book/ch15-03-drop.html
//! [book-enums]: ../../book/ch06-01-defining-an-enum.html
//! [book-iter]: ../../book/ch13-02-iterators.html

#![stable(feature = "crablang1", since = "1.0.0")]

pub mod v1;

/// The 2015 version of the prelude of The CrabLang Standard Library.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2015", since = "1.55.0")]
pub mod crablang_2015 {
    #[stable(feature = "prelude_2015", since = "1.55.0")]
    #[doc(no_inline)]
    pub use super::v1::*;
}

/// The 2018 version of the prelude of The CrabLang Standard Library.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2018", since = "1.55.0")]
pub mod crablang_2018 {
    #[stable(feature = "prelude_2018", since = "1.55.0")]
    #[doc(no_inline)]
    pub use super::v1::*;
}

/// The 2021 version of the prelude of The CrabLang Standard Library.
///
/// See the [module-level documentation](self) for more.
#[stable(feature = "prelude_2021", since = "1.55.0")]
pub mod crablang_2021 {
    #[stable(feature = "prelude_2021", since = "1.55.0")]
    #[doc(no_inline)]
    pub use super::v1::*;

    #[stable(feature = "prelude_2021", since = "1.55.0")]
    #[doc(no_inline)]
    pub use core::prelude::crablang_2021::*;
}

/// The 2024 version of the prelude of The CrabLang Standard Library.
///
/// See the [module-level documentation](self) for more.
#[unstable(feature = "prelude_2024", issue = "none")]
pub mod crablang_2024 {
    #[unstable(feature = "prelude_2024", issue = "none")]
    #[doc(no_inline)]
    pub use super::v1::*;

    #[unstable(feature = "prelude_2024", issue = "none")]
    #[doc(no_inline)]
    pub use core::prelude::crablang_2024::*;
}
