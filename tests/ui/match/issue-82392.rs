// https://github.com/crablang/crablang/issues/82329
// compile-flags: -Zunpretty=hir,typed
// check-pass

pub fn main() {
    if true {
    } else if let Some(a) = Some(3) {
    }
}
