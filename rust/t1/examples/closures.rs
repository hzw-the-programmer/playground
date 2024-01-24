#[derive(Copy, Clone)]
struct Foo {
    id: i32,
}

fn test1() {
    let mut f = Foo { id: 1 };
    println!("{:016p}", &f);

    {
        let mut c = || {
            println!("{:016p}", &f);
            f.id = 2;
        };
        println!();
        c();
        println!("{}", f.id);
    }

    {
        let mut c = move || {
            println!("{:016p}", &f);
            f.id = 3;
        };
        println!();
        c();
        println!("{}", f.id);
    }

    println!("{:016p}", &f);
}

struct FooTwo {
    id: i32,
}

fn test2() {
    let mut f = FooTwo { id: 1 };
    println!("{:016p}", &f);

    {
        let mut c = || {
            println!("{:016p}", &f);
            f.id = 2;
        };
        println!();
        c();
        println!("{}", f.id);
    }

    {
        let mut c = move || {
            println!("{:016p}", &f);
            f.id = 3;
        };
        println!();
        c();
        // println!("{}", f.id);
    }

    // println!("{:016p}", &f);
}

struct FooThree {
    id: i32,
    foo_2: FooTwo,
}

impl Drop for FooThree {
    fn drop(&mut self) {
        println!("FooThree {} drop", self.id);
    }
}

fn test3() {
    let mut f = FooThree {
        id: 1,
        foo_2: FooTwo { id: 2 },
    };
    println!("{:016p}", &f);

    {
        let mut c = || {
            println!("{:016p}", &f);
            f.id = 2;
        };
        println!();
        c();
        println!("{}", f.id);
    }

    {
        let mut c = move || {
            f.id = 3;
        };
        println!();
        c();
        println!("{}", f.id);
    }

    {
        let mut c = move || {
            let ff = &f;
        };
        println!();
        c();
        // println!("{}", f.id);
        // println!("{}", f.foo_2.id);
    }

    println!("finish");
}

fn main() {
    let tests = vec![test1, test2, test3];
    for (i, test) in tests.iter().enumerate() {
        print!("\n*** test {i} ***\n\n");
        test();
    }
}
