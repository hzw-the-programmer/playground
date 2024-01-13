#[derive(Debug)]
struct A {
    n: i32,
}

#[derive(Debug)]
struct B {
    n: i32,
}

impl From<A> for B {
    fn from(a: A) -> Self {
        B { n: a.n + 1 }
    }
}

fn main() {
    let a = A { n: 10 };
    let b = B::from(a);
    // println!("{:?}", a);
    println!("{:?}", b);

    let a = A { n: 10 };
    let b: B = a.into();
    // println!("{:?}", a);
    println!("{:?}", b);
}
