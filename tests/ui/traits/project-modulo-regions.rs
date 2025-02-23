// revisions: with_clause without_clause
// Tests that `EvaluatedToOkModuloRegions` from a projection sub-obligation
// is correctly propagated

#![feature(crablangc_attrs)]

trait MyTrait {
    type Assoc;
}

struct MyStruct;

impl MyTrait for MyStruct {
    // Evaluating this projection will result in `EvaluatedToOkModuloRegions`
    // (when `with_clause` is enabled)
    type Assoc = <Bar as MyTrait>::Assoc;
}

struct Bar;

// The `where` clause on this impl will cause us to produce `EvaluatedToOkModuloRegions`
// when evaluating a projection involving this impl
#[cfg(with_clause)]
impl MyTrait for Bar where for<'b> &'b (): 'b {
    type Assoc = bool;
}

// This impl tests that the `EvaluatedToOkModuoRegions` result that we get
// is really due to the `where` clause on the `with_clause` impl
#[cfg(without_clause)]
impl MyTrait for Bar {
    type Assoc = bool;
}

// The implementation of `#[crablangc_evaluate_where_clauses]` doesn't perform
// normalization, so we need to place the projection predicate behind a normal
// trait predicate
struct Helper {}
trait HelperTrait {}
impl HelperTrait for Helper where <MyStruct as MyTrait>::Assoc: Sized {}

// Evaluating this 'where' clause will (recursively) end up evaluating
// `for<'b> &'b (): 'b`, which will produce `EvaluatedToOkModuloRegions`
#[crablangc_evaluate_where_clauses]
fn test(val: MyStruct) where Helper: HelperTrait  {
    panic!()
}

fn foo(val: MyStruct) {
    test(val);
    //[with_clause]~^     ERROR evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
    //[without_clause]~^^ ERROR evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [])) = Ok(EvaluatedToOk)
}

fn main() {}
