// run-crablangfix
#![deny(warnings)]
#![feature(doc_notable_trait)]

#[doc(spotlight)]
//~^ ERROR unknown `doc` attribute `spotlight`
//~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
trait MyTrait {}
