struct D {
    i: u8,
}

impl Drop for D {
    fn drop(&mut self) {
        println!("{} drop", self.i);
    }
}

struct Dd {
    #[allow(dead_code)]
    d1: D,
    d2: D,
}

impl Drop for Dd {
    fn drop(&mut self) {
        println!("Dd drop");
    }
}

fn main() {
    let _d1 = D{i: 1};
    println!("_d1");
    let _d2 = D{i: 2};
    println!("_d2");
    let _dd = Dd{d1: D{i: 1}, d2: D{i: 2}};
    println!("_dd");
    let mut v = vec![D{i: 1}, D{i: 2}, D{i: 3}];
    println!("v");
    v.truncate(2);
    println!("truncate");

    {
        let dd = Dd{d1: D{i: 1}, d2: D{i: 2}};
        let d1 = dd.d1;
    }
    println!("main finish");
}
