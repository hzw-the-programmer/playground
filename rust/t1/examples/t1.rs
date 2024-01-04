struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo drop {}", self.id);
    }
}
fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("\ntest1\n");
    let mut _foo1 = Foo { id: 1 };
    let mut _foo2 = Foo { id: 2 };
}

fn test2() {
    println!("\ntest2\n");
    let v = vec![Foo { id: 1 }, Foo { id: 2 }];
    // let n: i32 = v;
    let mut i = &v[0];
    // let n: i32 = i;
    // i.id = 3;
    i = &v[1];
}

fn test3() {
    println!("\ntest3\n");
    let mut v = vec![Foo { id: 1 }, Foo { id: 2 }];
    // let n: i32 = v;
    let i = &mut v[0];
    // let n: i32 = i;
    i.id = 3;
    // i = &mut v[1];
}
