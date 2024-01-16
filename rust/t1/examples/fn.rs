fn test1() {
    println!("\ntest1\n");
    let square = |x| x * x;
    // let n: i32 = square;
    println!("{}", square(2));
    println!("{}", square(2));
}

fn call_with_one<F>(func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(1)
}

fn test2() {
    println!("\ntest2\n");
    let double = |x| x * 2;
    println!("{}", call_with_one(double));
}

fn test3() {
    println!("\ntest3\n");
    let mut x = 5;
    let mut square = || {
        // let n: f32 = x;
        x *= x
    };
    square();
    println!("{}", x);
}

fn do_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

fn test4() {
    println!("\ntest4\n");
    let mut x = 5;
    let double = || x *= 2;
    do_twice(double);
    println!("{}", x);
}

use t1::Foo;

fn consume_with_relish<F>(func: F)
where
    F: FnOnce() -> Foo,
{
    println!("consume_with_relish: begin");
    func();
    // let f = func();
    println!("consume_with_relish: end");
}

fn test5() {
    println!("\ntest5\n");
    let f = Foo { id: 1 };
    let func = || f;
    consume_with_relish(func);
    println!("finish");
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}
