pub fn test() {
    // test1();
    // test3();
    // test4();
    // test5();
    // test6();
    test7();
}

fn test1() {
    fn f_move<T: Fn()>(f: T) {
        f();
    }
    let c = || println!("closure");
    let f = c;
    f_move(c);
    c();
}

fn test2() {
    fn f_move<T: FnMut()>(f: &mut T) {
        f();
    }
    let mut i = 0;
    let mut c = || {
        println!("closure");
        i += 1
    };
    // let f = c;
    f_move(&mut c);
    // expected `&mut _`, found closure
    // f_move(c);
    // c();
}

fn test3() {
    fn f_move<T: FnOnce()>(f: T) {
        f();
    }
    let mut i = 0;
    let mut c = || {
        println!("closure");
        i += 1
    };
    // let i: i32 = &mut c;
    f_move(&mut c);
    c();
}

fn test4() {
    // cannot borrow `f` as mutable, as it is not declared as mutable
    // fn f_move<T: FnMut()>(f: T) {
    fn f_move<T: FnMut()>(mut f: T) {
        f();
    }
    let mut i = 0;
    let c = || {
        println!("closure");
        i += 1
    };
    f_move(c);
    // borrow of moved value: `c`
    // c();
}

fn test5() {
    // cannot borrow `f` as mutable, as it is not declared as mutable
    // fn f_move<T: FnMut()>(f: T) {
    fn f_move<T: FnMut()>(mut f: T) {
        f();
    }
    let mut i = 0;
    let mut c = || {
        println!("closure");
        i += 1
    };
    // expected a `FnMut()` closure, found `&{closure@examples\closure\src\move_test.rs:74:13: 74:15}`
    // f_move(&c);
    f_move(&mut c);
    c();
}

fn test6() {
    fn f_move<T: FnMut()>(ref mut f: T) {
        // expected `i32`, found `&mut T`
        // let i: i32 = f;
        f();
    }
    let mut i = 0;
    let c = || {
        println!("closure");
        i += 1
    };
    f_move(c);
    // borrow of moved value: `c`
    // c();
}

fn test7() {
    fn f_move<T: FnMut()>(ref mut f: T) {
        // expected `i32`, found `&mut T`
        // let i: i32 = f;
        f();
    }
    let mut i = 0;
    let mut c = || {
        println!("closure");
        i += 1
    };
    f_move(&mut c);
    c();
}

fn test8() {
    fn f_move<T: FnMut()>(f: &mut T) {
        f();
    }
    let mut i = 0;
    let mut c = || {
        println!("closure");
        i += 1
    };
    let g = &mut c;
    // borrow of moved value: `g`
    // let z = g;
    f_move(g);
    f_move(g);
}

fn test9() {
    fn f_move<T: FnMut()>(mut f: T) {
        f();
    }
    let mut i = 0;
    let mut c = || {
        println!("closure");
        i += 1
    };
    let g = &mut c;
    f_move(g);
    // use of moved value: `g`
    // f_move(g);
}
