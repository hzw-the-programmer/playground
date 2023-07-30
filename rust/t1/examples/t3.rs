struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo drop {}", self.id);
    }
}

fn main() {
    let _b = Box::new(Foo { id: 1 });
}
