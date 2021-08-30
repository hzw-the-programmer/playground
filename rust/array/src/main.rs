#[derive(Debug)]
struct E {
    f: i8,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("{} drop", self.f);
    }
}

fn main() {
    let x = vec![1, 2, 3];
    // for i in x {
    for i in &x {
        println!("{}", i);
    }
    println!("{:?}", x);

    let x = vec![E{f: 0}, E{f: 1}, E{f: 2}];
    // for i in x {
    for i in &x {
        println!("{:?}", i);
    }
    println!("{:?}", x);

    let e1 = E{f: 3};
    let x = vec![e1];
    // println!("{:?}", e1);
    println!("{:?}", x);
}
