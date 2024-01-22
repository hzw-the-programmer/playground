use std::pin::Pin;

struct Foo {
    id: i32,
}

impl Foo {
    fn id(&self) -> i32 {
        self.id
    }
}

fn main() {
    let mut f = Foo { id: 1 };
    {
        let mut p = Pin::new(&mut f);
        p.id = 2;
    }
    println!("{}", f.id);
}
