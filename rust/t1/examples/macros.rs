macro_rules! vec1 {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    }
}

fn main() {
    let v = vec1![1, 2, 3];
    println!("{:?}", v);

    let v = vec1!(1, 2, 3);
    println!("{:?}", v);
    let v = vec1! {1, 2, 3};
    println!("{:?}", v);

    // let v = vec1![1,, 2, 3];
    let v = vec1![1, 2, 3];
    println!("{:?}", v);
}
