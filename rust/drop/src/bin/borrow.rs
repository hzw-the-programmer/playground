use drop::Object;
use std::borrow::Borrow;
use std::ops::Deref;

fn main() {
    let tests: Vec<fn()> = vec![test0, test1, test2, test3];

    drop::tests(&tests);
}

fn test0() {
    let s = S {
        f1: Object { id: 1 },
        f2: Object { id: 2 },
    };
    let r: &S = s.borrow();
    println!("{:?}", r);
    let r1: &Object = s.borrow();
    println!("{:?}", r1);
}

fn test1() {
    let s = S {
        f1: Object { id: 1 },
        f2: Object { id: 2 },
    };
    let r = &s;
    let rr = &&s;
    let rrr = &&&s;
    let rrrr = &&&&s;

    /*
        T
    */
    s.fn1();

    /*
        &T
        T
    */
    r.fn1();

    /*
        &&T
        &T
        T
    */
    rr.fn1();
    rrr.fn1();
    s.fn2();
    r.fn2();
    rr.fn2();
    rrr.fn2();

    /*
        &&&&T
        &&&T
        &&T
        &T
        T
    */
    rrrr.fn2();
}

fn test2() {
    let s = S {
        f1: Object { id: 1 },
        f2: Object { id: 2 },
    };
    let r = &s;
    let rr = &&s;

    /*
        T
        Object
    */
    println!("{}", s.Id());
    println!("{}", r.Id());
    println!("{}", rr.Id());
}

fn test3() {}

#[derive(Debug)]
struct S {
    f1: Object,
    f2: Object,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("S dropped");
    }
}

impl Borrow<Object> for S {
    fn borrow(&self) -> &Object {
        &self.f1
    }
}

trait T {
    fn fn2(&self);
}

impl S {
    fn fn1(&self) {
        println!("S.fn1 called");
    }
}

impl T for S {
    fn fn2(&self) {
        println!("S.fn2 called");
    }
}

impl T for &S {
    fn fn2(&self) {
        println!("&S.fn2 called");
    }
}

impl T for &&S {
    fn fn2(&self) {
        println!("&&S.fn2 called");
    }
}

impl Deref for S {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.f2
    }
}
