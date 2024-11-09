pub fn test() {
    test1();
    test2();
}

fn test1() {
    let mut x = 2;
    {
        let mut square = || x *= x;
        square();
    }
    assert_eq!(x, 4);

    {
        let y = &mut x;
        (*y) *= *y;
    }
    assert_eq!(x, 16);

    {
        struct C<'a> {
            x: &'a mut i32,
        }
        // let c = C { x: &mut x };
        // (*c.x) *= *c.x;
        impl<'a> C<'a> {
            fn call_mut(&mut self) {
                *self.x *= *self.x;
            }
        }
        let mut c = C { x: &mut x };
        c.call_mut();
        assert_eq!(*c.x, 256);
    }
}

fn test2() {
    fn do_twice<F: FnMut()>(mut f: F) {
        f();
        f();
    }

    let mut x = 2;
    let add_one = || x += 1;
    do_twice(add_one);
    // borrow of moved value: `add_one`
    // add_one();
    assert_eq!(x, 4);
}
