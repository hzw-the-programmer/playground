fn main() {
    let f = Foo(vec![1, 2, 3]);
    println!("{}", f.len());
}

struct Foo<T>(Vec<T>);

use std::ops::Deref;

impl<T> Deref for Foo<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
