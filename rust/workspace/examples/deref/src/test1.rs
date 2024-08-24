#![allow(dead_code)]

struct Foo<'a> {
    i: &'a i32,
}

impl Foo<'_> {
    fn get(&self) -> &i32 {
        // let i: i32 = &self.i;
        &self.i
    }
}
pub fn test() {}
