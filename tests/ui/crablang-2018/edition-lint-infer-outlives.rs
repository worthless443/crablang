// run-crablangfix

#![allow(unused)]
#![deny(explicit_outlives_requirements)]

// Programmatically generated examples!
//
// Exercise outlives bounds for each of the following parameter/position
// combinations—
//
// • one generic parameter (T) bound inline
// • one parameter (T) with a where clause
// • two parameters (T and U), both bound inline
// • two parameters (T and U), one bound inline, one with a where clause
// • two parameters (T and U), both with where clauses
//
// —and for every permutation of 1 or 2 lifetimes to outlive and 0 or 1 trait
// bounds distributed among said parameters (subject to no where clause being
// empty and the struct having at least one lifetime).
//
// —and for each of tuple structs, enums and unions.

mod structs {
    use std::fmt::Debug;

    struct TeeOutlivesAy<'a, T: 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    struct TeeWhereOutlivesAy<'a, T> where T: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    struct TeeWhereOutlivesAyIsDebug<'a, T> where T: 'a + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    struct TeeWhereIsDebugOutlivesAy<'a, T> where T: Debug + 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    struct TeeWhereOutlivesAyBee<'a, 'b, T> where T: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    struct TeeWhereOutlivesAyBeeIsDebug<'a, 'b, T> where T: 'a + 'b + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    struct TeeWhereIsDebugOutlivesAyBee<'a, 'b, T> where T: Debug + 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    struct TeeYooOutlivesAy<'a, T, U: 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a U
    }

    struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a U
    }

    struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a U
    }

    struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T,
        yoo: U
    }

    struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a &'b U
    }

    struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a &'b U
    }

    struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a &'b U
    }

    struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
        yoo: U
    }

    struct TeeYooWhereOutlivesAy<'a, T, U> where U: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a U
    }

    struct TeeYooWhereOutlivesAyIsDebug<'a, T, U> where U: 'a + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a U
    }

    struct TeeYooWhereIsDebugOutlivesAy<'a, T, U> where U: Debug + 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a U
    }

    struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T,
        yoo: U
    }

    struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U> where U: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a &'b U
    }

    struct TeeYooWhereOutlivesAyBeeIsDebug<'a, 'b, T, U> where U: 'a + 'b + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a &'b U
    }

    struct TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U> where U: Debug + 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: T,
        yoo: &'a &'b U
    }

    struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
        yoo: U
    }

    struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U> where T: 'a, U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T,
        yoo: U
    }

    struct TeeWhereOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U> where T: 'a + 'b, U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
        yoo: U
    }

    struct BeeOutlivesAy<'a, 'b: 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b (),
    }

    struct BeeWhereOutlivesAy<'a, 'b> where 'b: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b (),
    }

    struct BeeOutlivesAyTee<'a, 'b: 'a, T> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    struct BeeWhereOutlivesAyTee<'a, 'b, T> where 'b: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    struct BeeWhereOutlivesAyTeeWhereBee<'a, 'b, T> where 'b: 'a, T: 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    struct BeeWhereOutlivesAyTeeWhereAyBee<'a, 'b, T> where 'b: 'a, T: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    struct BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T> where 'b: 'a, T: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }
}

mod tuple_structs {
    use std::fmt::Debug;

    struct TeeOutlivesAy<'a, T: 'a>(&'a T);
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug>(&'a T);
    //~^ ERROR outlives requirements can be inferred

    struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a>(&'a T);
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b>(&'a &'b T);
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug>(&'a &'b T);
    //~^ ERROR outlives requirements can be inferred

    struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b>(&'a &'b T);
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereOutlivesAy<'a, T>(&'a T) where T: 'a;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereOutlivesAyIsDebug<'a, T>(&'a T) where T: 'a + Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereIsDebugOutlivesAy<'a, T>(&'a T) where T: Debug + 'a;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereOutlivesAyBee<'a, 'b, T>(&'a &'b T) where T: 'a + 'b;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereOutlivesAyBeeIsDebug<'a, 'b, T>(&'a &'b T) where T: 'a + 'b + Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereIsDebugOutlivesAyBee<'a, 'b, T>(&'a &'b T) where T: Debug + 'a + 'b;
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooOutlivesAy<'a, T, U: 'a>(T, &'a U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug>(T, &'a U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a>(T, &'a U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug>(&'a T, U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b>(T, &'a &'b U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug>(T, &'a &'b U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b>(T, &'a &'b U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug>(&'a &'b T, U);
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooWhereOutlivesAy<'a, T, U>(T, &'a U) where U: 'a;
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooWhereOutlivesAyIsDebug<'a, T, U>(T, &'a U) where U: 'a + Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooWhereIsDebugOutlivesAy<'a, T, U>(T, &'a U) where U: Debug + 'a;
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U>(&'a T, U) where U: Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + 'b;
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooWhereOutlivesAyBeeIsDebug<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + 'b + Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) where U: Debug + 'a + 'b;
    //~^ ERROR outlives requirements can be inferred

    struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U>(&'a &'b T, U) where U: Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U>(&'a T, U) where T: 'a, U: Debug;
    //~^ ERROR outlives requirements can be inferred

    struct TeeWhereAyBeeYooWhereIsDebug<'a, 'b, T, U>(&'a &'b T, U) where T: 'a + 'b, U: Debug;
    //~^ ERROR outlives requirements can be inferred

    struct BeeOutlivesAy<'a, 'b: 'a>(&'a &'b ());
    //~^ ERROR outlives requirements can be inferred

    struct BeeWhereOutlivesAy<'a, 'b>(&'a &'b ()) where 'b: 'a;
    //~^ ERROR outlives requirements can be inferred

    struct BeeOutlivesAyTee<'a, 'b: 'a, T>(&'a &'b T);
    //~^ ERROR outlives requirements can be inferred

    struct BeeWhereOutlivesAyTee<'a, 'b, T>(&'a &'b T) where 'b: 'a;
    //~^ ERROR outlives requirements can be inferred

    struct BeeWhereOutlivesAyTeeWhereBee<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: 'b;
    //~^ ERROR outlives requirements can be inferred

    struct BeeWhereOutlivesAyTeeWhereAyBee<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: 'a + 'b;
    //~^ ERROR outlives requirements can be inferred

    struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug>(&'a &'b T);
    //~^ ERROR outlives requirements can be inferred

    struct BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: Debug;
    //~^ ERROR outlives requirements can be inferred
}

mod enums {
    use std::fmt::Debug;

    enum TeeOutlivesAy<'a, T: 'a> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a T },
    }

    enum TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
        //~^ ERROR outlives requirements can be inferred
        V(&'a T),
    }

    enum TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a T },
        W,
    }

    enum TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
        W,
    }

    enum TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b T },
    }

    enum TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
    }

    enum TeeWhereOutlivesAy<'a, T> where T: 'a {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a T },
        W,
    }

    enum TeeWhereOutlivesAyIsDebug<'a, T> where T: 'a + Debug {
        //~^ ERROR outlives requirements can be inferred
        V(&'a T),
        W,
    }

    enum TeeWhereIsDebugOutlivesAy<'a, T> where T: Debug + 'a {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a T },
    }

    enum TeeWhereOutlivesAyBee<'a, 'b, T> where T: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
    }

    enum TeeWhereOutlivesAyBeeIsDebug<'a, 'b, T> where T: 'a + 'b + Debug {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b T },
        W,
    }

    enum TeeWhereIsDebugOutlivesAyBee<'a, 'b, T> where T: Debug + 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
        W,
    }

    enum TeeYooOutlivesAy<'a, T, U: 'a> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T },
        W(&'a U),
    }

    enum TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T, yoo: &'a U },
        W,
    }

    enum TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
        //~^ ERROR outlives requirements can be inferred
        V(T, &'a U),
        W,
    }

    enum TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a T },
        W(U),
    }

    enum TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T, yoo: &'a &'b U },
        W,
    }

    enum TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
        //~^ ERROR outlives requirements can be inferred
        V(T, &'a &'b U),
        W,
    }

    enum TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T, yoo: &'a &'b U },
        W,
    }

    enum TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T, U),
        W,
    }

    enum TeeYooWhereOutlivesAy<'a, T, U> where U: 'a {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T },
        W(&'a U),
    }

    enum TeeYooWhereOutlivesAyIsDebug<'a, T, U> where U: 'a + Debug {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T, yoo: &'a U },
        W,
    }

    enum TeeYooWhereIsDebugOutlivesAy<'a, T, U> where U: Debug + 'a {
        //~^ ERROR outlives requirements can be inferred
        V(T, &'a U),
        W,
    }

    enum TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a T },
        W(U),
    }

    enum TeeYooWhereOutlivesAyBee<'a, 'b, T, U> where U: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T, yoo: &'a &'b U },
        W,
    }

    enum TeeYooWhereOutlivesAyBeeIsDebug<'a, 'b, T, U> where U: 'a + 'b + Debug {
        //~^ ERROR outlives requirements can be inferred
        V(T, &'a &'b U),
        W,
    }

    enum TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U> where U: Debug + 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        V { tee: T },
        W(&'a &'b U),
    }

    enum TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b T, yoo: U },
        W,
    }

    enum TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U> where T: 'a, U: Debug {
        //~^ ERROR outlives requirements can be inferred
        V(&'a T, U),
        W,
    }

    enum TeeWhereOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U> where T: 'a + 'b, U: Debug {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b T },
        W(U),
    }

    enum BeeOutlivesAy<'a, 'b: 'a> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b () },
    }

    enum BeeWhereOutlivesAy<'a, 'b> where 'b: 'a {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b ()),
    }

    enum BeeOutlivesAyTee<'a, 'b: 'a, T> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b T },
        W,
    }

    enum BeeWhereOutlivesAyTee<'a, 'b, T> where 'b: 'a {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
        W,
    }

    enum BeeWhereOutlivesAyTeeWhereBee<'a, 'b, T> where 'b: 'a, T: 'b {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
    }

    enum BeeWhereOutlivesAyTeeWhereAyBee<'a, 'b, T> where 'b: 'a, T: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
        W,
    }

    enum BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
        //~^ ERROR outlives requirements can be inferred
        V { tee: &'a &'b T },
    }

    enum BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T> where 'b: 'a, T: Debug {
        //~^ ERROR outlives requirements can be inferred
        V(&'a &'b T),
    }
}

mod unions {
    use std::fmt::Debug;

    union TeeOutlivesAy<'a, T: 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    union TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    union TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    union TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    union TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    union TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    union TeeWhereOutlivesAy<'a, T> where T: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    union TeeWhereOutlivesAyIsDebug<'a, T> where T: 'a + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    union TeeWhereIsDebugOutlivesAy<'a, T> where T: Debug + 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T
    }

    union TeeWhereOutlivesAyBee<'a, 'b, T> where T: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    union TeeWhereOutlivesAyBeeIsDebug<'a, 'b, T> where T: 'a + 'b + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    union TeeWhereIsDebugOutlivesAyBee<'a, 'b, T> where T: Debug + 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T
    }

    union TeeYooOutlivesAy<'a, T, U: 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a U
    }

    union TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a U
    }

    union TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a U
    }

    union TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T,
        yoo: *const U
    }

    union TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a &'b U
    }

    union TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a &'b U
    }

    union TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a &'b U
    }

    union TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
        yoo: *const U
    }

    union TeeYooWhereOutlivesAy<'a, T, U> where U: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a U
    }

    union TeeYooWhereOutlivesAyIsDebug<'a, T, U> where U: 'a + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a U
    }

    union TeeYooWhereIsDebugOutlivesAy<'a, T, U> where U: Debug + 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a U
    }

    union TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T,
        yoo: *const U
    }

    union TeeYooWhereOutlivesAyBee<'a, 'b, T, U> where U: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a &'b U
    }

    union TeeYooWhereOutlivesAyBeeIsDebug<'a, 'b, T, U> where U: 'a + 'b + Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a &'b U
    }

    union TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U> where U: Debug + 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: *const T,
        yoo: &'a &'b U
    }

    union TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
        yoo: *const U
    }

    union TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U> where T: 'a, U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a T,
        yoo: *const U
    }

    union TeeWhereOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U> where T: 'a + 'b, U: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
        yoo: *const U
    }

    union BeeOutlivesAy<'a, 'b: 'a> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b (),
    }

    union BeeWhereOutlivesAy<'a, 'b> where 'b: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b (),
    }

    union BeeOutlivesAyTee<'a, 'b: 'a, T> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    union BeeWhereOutlivesAyTee<'a, 'b, T> where 'b: 'a {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    union BeeWhereOutlivesAyTeeWhereBee<'a, 'b, T> where 'b: 'a, T: 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    union BeeWhereOutlivesAyTeeWhereAyBee<'a, 'b, T> where 'b: 'a, T: 'a + 'b {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    union BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }

    union BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T> where 'b: 'a, T: Debug {
        //~^ ERROR outlives requirements can be inferred
        tee: &'a &'b T,
    }
}


// But outlives inference for 'static lifetimes is under a separate
// feature-gate for now
// (https://github.com/crablang/crablang/issues/44493#issuecomment-407846046).
struct StaticRef<T: 'static> {
    field: &'static T
}

struct TrailingCommaInWhereClause<'a, T, U>
where
    T: 'a,
    U: 'a,
    //~^ ERROR outlives requirements can be inferred
{
    tee: T,
    yoo: &'a U
}

fn main() {}
