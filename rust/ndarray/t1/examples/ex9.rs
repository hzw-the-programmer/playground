use ndarray::prelude::*;
use ndarray::Array;
use std::iter::FromIterator;
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::Uniform;

fn main() {
    // Or you may use ndarray_rand crate to generate random arrays
    // let a = Array::random((2, 5), Uniform::new(0., 10.));

    let a = array![[3., 7., 3., 4.], [1., 4., 2., 2.], [7., 2., 4., 9.]];

    println!("a = \n{:?}\n", a);

    // use trait FromIterator to flatten a matrix to a vector
    let b = Array::from_iter(a.iter());
    println!("b = \n{:?}\n", b);

    let c = b.into_shape([6, 2]).unwrap(); // consume b and generate c with new shape
    println!("c = \n{:?}", c);
}
