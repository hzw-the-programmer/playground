fn main() {
    let tests: Vec<fn()> = vec![test0, test1, test2, test3];

    drop::tests(&tests);
}

fn test0() {
    let s = S { f1: 1 };
    /*
        T
        &T
        &mut T
    */
    s.func1();
    println!("{}", s.f1);
    /*
        &T
        &&T
        &mut &T
        T
        &T
        &mut T
    */
    let r = &S { f1: 1 };
    r.func1();
    /*
        &&T
        &&&T
        &mut &&T
        &T
        &&T
        &mut T
        T
        &T
        &mut T
    */
    let rr = &&S { f1: 1 };
    rr.func1();

    /*
        T
        &T
        &mut T
    */
    s.func2();
    r.func2();
}

fn test1() {}

fn test2() {}

fn test3() {}

#[derive(Copy, Clone)]
struct S {
    f1: i32,
}

impl S {
    fn func1(self) {
        println!("func1(self) called");
    }

    fn func2(&self) {
        println!("func2(&self) called");
    }
}
