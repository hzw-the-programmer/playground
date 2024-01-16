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

fn test1() {
    println!("\ntest1\n");
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    assert_eq!(Point { x: 3, y: 3 }, p1 + p2);
    println!("{:?} {:?}", p1, p2);
}

#[derive(PartialEq, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Add for &Point2 {
    type Output = Point2;

    fn add(self, rhs: Self) -> Self::Output {
        // let n: i32 = self;
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn test2() {
    println!("\ntest2\n");
    let p1 = &Point2 { x: 1, y: 0 };
    let p2 = &Point2 { x: 2, y: 3 };
    assert_eq!(Point2 { x: 3, y: 3 }, p1 + p2);
    println!("{:?} {:?}", p1, p2);
}

fn main() {
    test1();
    test2();
}
