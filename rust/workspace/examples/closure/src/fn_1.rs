pub fn test() {
    let tests = [test1, test2, test3, test4];
    for (i, test) in tests.iter().enumerate() {
        println!("\ntest{}: begin", i + 1);
        test();
        println!("test{}: end", i + 1);
    }
}

fn test1() {
    let f = |x: i32| x + 1;
    // expected `i32`, found closure
    // let i: i32 = f;

    fn1(f);
    fn2(f);

    // let f: &dyn Fn(i32) -> i32 = &f;
    // let f: fn(i32) -> i32 = f;
    // expected `i32`, found fn pointer
    // let i: i32 = f;
}

fn test2() {
    fn f1(x: i32) -> i32 {
        x + 1
    }
    // expected `i32`, found fn item
    // let i: i32 = f1;

    fn1(f1);
    fn2(f1);

    // let f: &dyn Fn(i32) -> i32 = &f1;
    // let f: fn(i32) -> i32 = f1;
    // expected `i32`, found fn pointer
    // let i: i32 = f;
}

fn test3() {
    let env = 10;
    let c = |x| x + env;
    // expected `i32`, found closure
    // let i: i32 = c;
    let f: &dyn Fn(i32) -> i32 = &c;
    // expected fn pointer, found closure
    // closures can only be coerced to `fn` types if they do not capture any variables
    // let f: fn(i32) -> i32 = c;
}

fn test4() {}

fn fn1<F: Fn(i32) -> i32>(f: F) {
    f(1);
}
fn fn2(f: fn(i32) -> i32) {
    f(2);
}
