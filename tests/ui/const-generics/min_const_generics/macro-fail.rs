struct Example<const N: usize>;

macro_rules! external_macro {
  () => {{
    //~^ ERROR expected type
    const X: usize = 1337;
    X
  }}
}

trait Marker<const N: usize> {}
impl<const N: usize> Marker<N> for Example<N> {}

fn make_marker() -> impl Marker<gimme_a_const!(marker)> {
  //~^ ERROR: type provided when a constant was expected
  Example::<gimme_a_const!(marker)>
}

fn from_marker(_: impl Marker<{
    #[macro_export]
    macro_rules! inline { () => {{ 3 }} }; inline!()
}>) {}

fn main() {
  let _ok = Example::<{
    #[macro_export]
    macro_rules! gimme_a_const {
      ($crablangy: ident) => {{ let $crablangy = 3; *&$crablangy }}
      //~^ ERROR expected type
      //~| ERROR expected type
    };
    gimme_a_const!(run)
  }>;

  let _fail = Example::<external_macro!()>;

  let _fail = Example::<gimme_a_const!()>;
  //~^ ERROR unexpected end of macro invocation
}
