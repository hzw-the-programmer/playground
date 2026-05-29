#[derive(Debug)]
pub struct Foo;
#[derive(Debug)]
pub struct Bar;
#[derive(Debug)]
pub struct Baz;

impl From<Baz> for Bar {
    fn from(_baz: Baz) -> Self {
        Bar
    }
}

impl From<i32> for Bar {
    fn from(_: i32) -> Self {
        Bar
    }
}
