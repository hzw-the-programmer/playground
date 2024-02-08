use ndarray::prelude::*;
use ndarray::Array;
fn main() {
    let a = Array::<f64, _>::zeros((3, 2, 4).f());
    println!("{:?}", a);
}
