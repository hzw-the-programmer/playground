fn main() {
    let f1 = Object { id: 1 };
    let f2 = Object { id: 2 };
    let s = S { f1, f2 };

    let S { f1, f2 } = s;
    println!("{:?} {:?}", f1, f2);
    println!("{:?}", s);
    println!("finish");
}