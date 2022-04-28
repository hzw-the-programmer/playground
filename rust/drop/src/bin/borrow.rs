use drop::Object;
use std::borrow::Borrow;

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

fn test1() {}
fn test2() {}
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
        return &self.f1;
    }
}
