use ndarray::prelude::*;
use ndarray::Array;

fn main() {
    let mut a = Array::range(0., 4., 1.).into_shape([2, 2]).unwrap();
    let b = a.clone();

    println!("a = \n{}\n", a);
    println!("b clone of a = \n{}\n", a);

    a.slice_mut(s![1, 1]).fill(1234.);

    println!("a updated...");
    println!("a = \n{}\n", a);
    println!("b clone of a = \n{}\n", b);
}
