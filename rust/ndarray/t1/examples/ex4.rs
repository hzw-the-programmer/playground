use ndarray::prelude::*;
use ndarray::Array;
use std::f64::INFINITY as inf;

fn main() {
    let a = array![[10., 20., 30., 40.,],];
    let b = Array::range(0., 4., 1.); // [0., 1., 2., 3, ]

    println!("{:?}", a);
    println!("{:?}", b);

    assert_eq!(&a + &b, array![[10., 21., 32., 43.,]]); // Allocates a new array. Note the explicit `&`.
    assert_eq!(&a - &b, array![[10., 19., 28., 37.,]]);
    assert_eq!(&a * &b, array![[0., 20., 60., 120.,]]);
    assert_eq!(&a / &b, array![[inf, 20., 15., 13.333333333333334,]]);
}
