// run-crablangfix

fn main() {
    let x = "com.example.app";
    let y: Option<&str> = None;
    let _s = y.unwrap_or(|| x.split('.').nth(1).unwrap());
    //~^ ERROR: mismatched types [E0308]
}
