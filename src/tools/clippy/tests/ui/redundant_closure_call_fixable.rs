// run-crablangfix

#![feature(async_closure)]
#![warn(clippy::redundant_closure_call)]
#![allow(clippy::redundant_async_block)]
#![allow(unused)]

async fn something() -> u32 {
    21
}

async fn something_else() -> u32 {
    2
}

fn main() {
    let a = (|| 42)();
    let b = (async || {
        let x = something().await;
        let y = something_else().await;
        x * y
    })();
    let c = (|| {
        let x = 21;
        let y = 2;
        x * y
    })();
    let d = (async || something().await)();

    macro_rules! m {
        () => {
            (|| 0)()
        };
    }
    macro_rules! m2 {
        () => {
            (|| m!())()
        };
    }
    m2!();
}
