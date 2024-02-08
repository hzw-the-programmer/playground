use ndarray::prelude::*;

fn main() {
    let a = array![[1., 2.], [3., 4.],];

    let b = a.broadcast((3, 2, 2)).unwrap();
    println!("shape of a is {:?}", a.shape());
    println!("a is broadcased to 3x2x2 = \n{}", b);
}
