pub fn test() {
    // test1();
    // test2();
    // test4();
    // test5();
    // test7();
    test8();
}

fn test1() {
    let a = [1, 2, 3];
    // expected `i32`, found `&[{integer}; 3]`
    // let i: i32 = &a;

    // expected `i32`, found `[{integer}]`
    // let i: i32 = a[..];

    // expected `i32`, found `&[{integer}]`
    // let i: i32 = &a[..];

    // `s` is a `&` reference, so the data it refers to cannot be written
    // let s = &a[..];
    // s[0] = 2;

    // `s` is a `&` reference, so the data it refers to cannot be written
    // let mut s = &a[..];
    // s[0] = 2;

    // cannot borrow as mutable
    // let s = &mut a[..];
    // s[0] = 2;
}

fn test2() {
    let mut a = [1, 2, 3];
    let s = &mut a[..];
    s[0] = 2;
    // cannot assign twice to immutable variable
    // s = &mut a[..];

    let mut s = &mut a[..];
    let s1 = &mut s;
    s1[0] = 10;
    let mut b = [1, 2, 3, 4];
    *s1 = &mut b[..];
    println!("{:?}", s);

    // cannot assign twice to immutable variable
    // s1 = &mut s;
}

fn test3() {
    let mut a = [1, 2, 3];
    let s = &mut a[..];
    s[0] = 2;

    let mut b = [1, 2, 3, 4];
    // doesn't have a size known at compile-time
    // *s = b[..];
    println!("{:?}", s);
}

fn test4() {
    let a = [1, 2, 3];
    let mut s = &a[..];
    // `s` is a `&` reference, so the data it refers to cannot be written
    // s[0] = 2;
    let s1 = &mut s;
    // cannot assign to `s1[_]`, which is behind a `&` reference
    // s1[0] = 10;
    *s1 = &a[2..];
    println!("{:?}", s);
}

fn test5() {
    {
        let a = [1, 2, 3, 4];
        let s = &a[..];
        let (p1, p2) = s.split_at(2);
        println!("{:?}", s);
        println!("{:?}", p1);
        println!("{:?}", p2);
    }

    {
        let a = [1, 2, 3, 4];
        let mut s = &a[..];
        let s1 = &mut s;
        // expected `i32`, found `&mut &[{integer}]`
        // let i: i32 = &mut s;
        let (p1, p2) = s1.split_at(2);
        println!("{:?}", s);
        println!("{:?}", p1);
        println!("{:?}", p2);
    }
}

fn test6() {
    let mut a = [1, 2, 3, 4];
    // let s = &mut a[..];
    // `*s` is borrowed here
    // let (p1, p2) = s.split_at_mut(2);
    // `a` is borrowed here
    let (p1, p2) = a.split_at_mut(2);
    // println!("{:?}", s);
    // use of borrowed `*s`
    // s[0] = 1;
    // use of borrowed `a`
    // a[0] = 1;
    // borrow later used here
    println!("{:?}", p1);
    println!("{:?}", p2);
    // s[0] = 1;
    a[0] = 1;
}

fn test7() {
    {
        let a = [1, 2, 3];
        println!("{}", std::any::type_name_of_val(&a));
    }

    {
        let mut a = [1, 2, 3];
        println!("{}", std::any::type_name_of_val(&a));
    }

    {
        let a = [1, 2, 3];
        let s = &a[..];
        println!("{}", std::any::type_name_of_val(&s));
    }

    {
        let mut a = [1, 2, 3];
        let s = &mut a[..];
        println!("{}", std::any::type_name_of_val(&s));
    }

    {
        let a = [1, 2, 3];
        let mut s = &a[..];
        println!("{}", std::any::type_name_of_val(&s));
    }

    {
        let mut a = [1, 2, 3];
        let mut s = &mut a[..];
        println!("{}", std::any::type_name_of_val(&s));
    }

    {
        let a = [1, 2, 3];
        let s = &a[..];
        let s1 = &s;
        println!("{}", std::any::type_name_of_val(&s1));
    }

    {
        let mut a = [1, 2, 3];
        let s = &mut a[..];
        let s1 = &s;
        println!("{}", std::any::type_name_of_val(&s1));
    }

    {
        let a = [1, 2, 3];
        let mut s = &a[..];
        let s1 = &s;
        println!("{}", std::any::type_name_of_val(&s1));
    }

    {
        let a = [1, 2, 3];
        let mut s = &a[..];
        let s1 = &mut s;
        println!("{}", std::any::type_name_of_val(&s1));
    }

    {
        let mut a = [1, 2, 3];
        let mut s = &mut a[..];
        let s1 = &s;
        println!("{}", std::any::type_name_of_val(&s1));
    }

    {
        let mut a = [1, 2, 3];
        let mut s = &mut a[..];
        let s1 = &mut s;
        println!("{}", std::any::type_name_of_val(&s1));
    }
}

fn test8() {
    use std::io::Read;
    let a = [1, 2, 3];
    let (p1, p2) = a.split_at(1);
    let mut b = [0; 10];
    // method not found in `[{integer}; 3]`
    // let _ = a.read(&mut b);
    // cannot borrow as mutable
    // let s = &a[..];
    let mut s = &a[..];
    let _ = s.read(&mut b);
    println!("{:?}", s);
    println!("{:?}", b);
}
