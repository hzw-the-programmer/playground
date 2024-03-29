use std::pin::Pin;

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

impl Foo {
    fn work_1(self: Pin<&mut Self>) {
        println!("Foo.work_1");
    }

    fn work_2(self: Pin<&Self>) {
        println!("Foo.work_2");
    }
}

fn test1() {
    print!("\ntest1\n\n");

    let mut f = Foo { id: 1 };
    // f.work_1();
    let pinned_mut = Pin::new(&mut f);
    // let pinned_mut = Pin { pointer: &mut f };
    pinned_mut.work_1();
    // pinned_mut.work_2();

    let pinned = Pin::new(&f);
    // pinned.work_1();
    pinned.work_2();

    println!("finish");
}

fn main() {
    test1();
}
