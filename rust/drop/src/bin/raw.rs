use drop::Object;

fn test0() {
    let o = Object { id: 1 };
    let p = &o as *const Object;
    let p1 = p as *mut Object;
    let o2 = unsafe { &mut *p1 };
    o2.id = 2;
    let o1 = unsafe { &*p };
    // let o1 = unsafe{ *p };
}

fn test1() {
    let mut o = Object { id: 1 };
    let p = &mut o as *mut Object;
    let o1 = unsafe { &mut *p };
    // let o1 = unsafe { *p };
}

fn test2() {
    let o = Object { id: 1 };
    let p = &o as *const Object as *mut Object;
    // let p = &o as *const Object;
    unsafe {
        *p = Object { id: 2 };
    }
    println!("finish")
}

fn test3() {
    let p;
    {
        let o = Object { id: 1 };
        // p = &o;
        p = &o as *const Object;
    }
    println!("{:?}", p);
    unsafe {
        println!("{:?}", *p);
    }
}

fn test4_h() -> *const Object {
    let o = Object { id: 1 };
    return &o as *const Object;
}

fn test4() {
    let p = test4_h();
    println!("{:?}", p);
    unsafe {
        println!("{:?}", *p);
    }
}

fn test5() {}
fn test6() {}
fn test7() {}
fn test8() {}
fn test9() {}
fn test10() {}

struct NonNull<T> {
    ptr: *const T,
}

fn main() {
    let tests: Vec<fn()> = vec![
        test0, test1, test2, test3, test4, test5, test6, test7, test8, test9,
        test10, // test11,
               // test12, test13, test14, test15, test16, test17, test18, test19, test20, test21, test22,
               // test23, test24,
    ];

    drop::tests(&tests);
}
