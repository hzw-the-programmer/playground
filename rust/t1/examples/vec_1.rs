fn main() {
    let foos = vec![Foo { id: 1 }, Foo { id: 2 }, Foo { id: 3 }];

    for _foo in foos {
        println!("in loop");
    }
}

struct Foo {
    id: u64,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} dropped", self.id);
    }
}
