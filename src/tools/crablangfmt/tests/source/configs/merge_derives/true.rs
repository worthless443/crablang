// crablangfmt-merge_derives: true
// Merge multiple derives to a single one.

#[bar]
#[derive(Eq, PartialEq)]
#[foo]
#[derive(Debug)]
#[foobar]
#[derive(Copy, Clone)]
pub enum Foo {}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[foobar]
#[derive(Copy, Clone)]
pub enum Bar {}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum FooBar {}

mod foo {
#[bar]
#[derive(Eq, PartialEq)]
#[foo]
#[derive(Debug)]
#[foobar]
#[derive(Copy, Clone)]
pub enum Foo {}
}

mod bar {
#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[foobar]
#[derive(Copy, Clone)]
pub enum Bar {}
}

mod foobar {
#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum FooBar {}
}
