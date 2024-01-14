struct A {
    i: i32,
}

#[derive(Debug)]
struct B {
    i: i32,
}

impl From<A> for B {
    fn from(a: A) -> Self {
        B { i: a.i + 1 }
    }
}

fn main() {
    let a1 = A { i: 1 };
    let b1 = B::from(a1);
    println!("{:?}", b1);

    let a2 = A { i: 2 };
    let b2: B = a2.into();
    println!("{:?}", b2);
}
