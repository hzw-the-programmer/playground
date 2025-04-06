mod my_box;
use my_box::MyBox;
mod my_global_alloc;
use my_global_alloc::COUNTER;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    test1();
}

fn test1() {
    println!("allocated bytes before main: {}", COUNTER.load(Relaxed));

    println!("allocated bytes before block: {}", COUNTER.load(Relaxed));
    {
        let _mb = MyBox::new(Foo { id: 1 });
    }
    println!("allocated bytes after block: {}", COUNTER.load(Relaxed));
}

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Drop Foo {}", self.id);
    }
}
