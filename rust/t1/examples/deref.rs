use std::ops::Deref;

#[derive(PartialEq, Debug)]
struct Foo {
    id: i32,
}

fn main() {
    test1();
}

fn test1() {
    let x = 5;
    let y = &x;
    assert_eq!(x, *y);

    let x = 5;
    let y = Box::new(x);
    // assert_eq!(x, y.deref());
    assert_eq!(x, *(y.deref()));
    assert_eq!(x, *y);

    let x = 5;
    let y = MyBox::new(x);
    // assert_eq!(x, y);
    // assert_eq!(x, y.deref());
    assert_eq!(x, *y.deref());
    assert_eq!(x, *y);

    hello(&MyBox::new(String::from("hzw")));

    let f = Foo { id: 1 };
    let r = &f;
    // let b = *r;

    let mb = MyBox(Foo { id: 1 });
    // let b = *mb;
    assert_eq!(Foo { id: 1 }, *mb);
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello {name}");
}
