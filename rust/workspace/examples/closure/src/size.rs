pub fn test() {
    test1();
}

fn type_size<T>(c: T) -> usize {
    size_of::<T>()
}

fn test1() {
    let c = || {
        println!("hello");
    };
    assert_eq!(size_of_val(&c), 0);
    assert_eq!(type_size(c), 0);

    let c = |a: i8| {
        println!("{:?}", a);
    };
    assert_eq!(size_of_val(&c), 0);
    assert_eq!(type_size(c), 0);

    {
        let a = 0;
        let c = || {
            println!("{:?}", a);
        };
        assert_eq!(size_of_val(&c), 8);
        assert_eq!(type_size(c), 8);
    }

    {
        let a = 0;
        let c = move || {
            println!("{:?}", a);
        };
        assert_eq!(size_of_val(&c), 4);
        assert_eq!(type_size(c), 4);
    }

    {
        let a = 0i8;
        let c = move || {
            println!("{:?}", a);
        };
        assert_eq!(size_of_val(&c), 1);
        assert_eq!(type_size(c), 1);
    }
}
