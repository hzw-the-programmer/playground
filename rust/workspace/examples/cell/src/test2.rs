use std::cell::RefCell;
use std::rc::Rc;

pub fn test() {
    test1();
}

fn test1() {
    let rc = Rc::new(RefCell::new(Foo(1)));
    let b = rc.borrow_mut();
    let rc2 = rc.clone();
    let c = rc2.borrow();
}

struct Foo(i32);
