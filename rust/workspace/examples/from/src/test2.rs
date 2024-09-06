use core::task::Poll;

pub fn test() {
    let r = f1();
    println!("{:?}", r);
}

fn f1() -> Poll<Result<Foo, Bar>> {
    // let r = Ok(Foo(1));
    let r = Err(Bar(1));
    let r = r?;
    Poll::Ready(Ok(Foo(1)))
}

#[derive(Debug)]
struct Foo(i32);
#[derive(Debug)]
struct Bar(i32);
