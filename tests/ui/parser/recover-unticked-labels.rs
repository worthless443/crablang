// run-crablangfix

fn main() {
    'label: loop { break label };    //~ error: cannot find value `label` in this scope
    'label: loop { break label 0 };  //~ error: expected a label, found an identifier
    'label: loop { continue label }; //~ error: expected a label, found an identifier
}
