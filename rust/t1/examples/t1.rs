struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo drop {}", self.id);
    }
}
fn main() {
    let mut _foo1 = Foo { id: 1 };
    let mut _foo2 = Foo { id: 2 };
}
