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
    let _d2 = D{i: 2};
    let _dd = Dd{d1: D{i: 1}, d2: D{i: 2}};
    println!("main finish");
}
