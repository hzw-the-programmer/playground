use std::ptr;
use t1::Foo;

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

fn test1() {
    println!("\ntest1\n");
    let mut a = [1, 2, 3];
    // let i: i32 = a;
    let p = a.as_mut_ptr();
    // let i: i32 = p;
    unsafe {
        println!("{}", *p.offset(1));
        println!("{}", *p.offset(2));
    }
}

fn test2() {
    println!("\ntest2\n");
    let f = Foo { id: 1 };
    let p = &f as *const Foo;
    let mut f1 = unsafe { ptr::read(p) };
    f1.id = 2;
}

fn test3() {
    println!("\ntest3\n");
    let mut f = Foo { id: 1 };
    let p = &mut f as *mut Foo;
    unsafe {
        ptr::write(p, Foo { id: 2 });
    };
}

fn test4() {
    println!("\ntest4\n");
    let mut f = Foo { id: 1 };
    println!("{:?}", &f as *const Foo);
    f = Foo { id: 2 };
    println!("{:?}", &f as *const Foo);
    println!("finish");
}

fn test5() {
    println!("\ntest5\n");
    let f = Foo { id: 1 };
    println!("{:?}", &f as *const Foo);
    let f = Foo { id: 2 };
    println!("{:?}", &f as *const Foo);
    println!("finish");
}
