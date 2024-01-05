struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

trait Fun {
    fn fun(&self);
}

impl<T> Point<T>
where
    T: Fun,
{
    fn call_fun(&self) {}
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("p.x={}", p.x());
    // println!("{}", p.distance_from_origin());
    // p.call_fun();

    let p1 = Foo { x: 4, y: 1.0 };
    // let n: i32 = p1;
    let p2 = Foo { x: "str", y: 'c' };
    // let n: i32 = p2;
    p1.mixup(&p2);
}

// struct Foo<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Foo<X1, Y1> {
//     fn mixup<X2, Y2>(&self, other: &Foo<X2, Y2>) -> Foo<X1, Y2> {
//         Foo {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// struct Foo<X1, Y1>
// where
//     X1: Copy,
// {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Foo<X1, Y1>
// where
//     X1: Copy,
// {
//     fn mixup<X2, Y2>(&self, other: &Foo<X2, Y2>) -> Foo<X1, Y2>
//     where
//         X2: Copy,
//         Y2: Copy,
//     {
//         Foo {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

struct Foo<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Foo<X1, Y1>
where
    X1: Copy,
{
    fn mixup<X2, Y2>(&self, other: &Foo<X2, Y2>) -> Foo<X1, Y2>
    where
        Y2: Copy,
    {
        Foo {
            x: self.x,
            y: other.y,
        }
    }
}
