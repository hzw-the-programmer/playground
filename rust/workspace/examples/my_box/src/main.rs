mod my_box;
use my_box::MyBox;

fn main() {
    test1();
}

fn test1() {
    let mb = MyBox::new(Foo { id: 1 });
}

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Drop Foo {}", self.id);
    }
}
