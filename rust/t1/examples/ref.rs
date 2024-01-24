trait Work {
    fn work(self);
}

trait WorkRef {
    fn work_ref(&self);
}

struct Foo {
    id: i32,
}

impl Work for Foo {
    fn work(self) {
        println!("Foo.work");
    }
}

impl WorkRef for Foo {
    fn work_ref(&self) {
        println!("Foo.work_ref");
    }
}

impl Work for &Foo {
    fn work(self) {
        println!("&Foo.work");
    }
}

impl WorkRef for &Foo {
    fn work_ref(&self) {
        println!("&Foo.work_ref");
    }
}

impl Work for &mut Foo {
    fn work(self) {
        println!("&mut Foo.work");
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

fn test1() {
    print!("\ntest1\n\n");

    let f = Foo { id: 1 };
    f.work_ref();
    f.work();

    println!("finish");
}

fn test2() {
    print!("\ntest2\n\n");

    let f = &Foo { id: 1 };
    f.work_ref();
    f.work();

    println!("finish");
}

fn test3() {
    print!("\ntest3\n\n");

    let f = &&Foo { id: 1 };
    f.work_ref();
    f.work();

    println!("finish");
}

fn test4() {
    print!("\ntest4\n\n");

    let f = &mut Foo { id: 1 };
    f.work_ref();
    f.work();

    println!("finish");
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}
