use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    assert_eq!(Point { x: 3, y: 3 }, p1 + p2);
    println!("{:?} {:?}", p1, p2);
}
