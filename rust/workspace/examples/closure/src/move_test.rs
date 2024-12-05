pub fn test() {
    test1();
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
    // c();
}
