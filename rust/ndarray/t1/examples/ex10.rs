use ndarray::prelude::*;
use ndarray::{concatenate, stack, Axis};

fn main() {
    let a = array![[3., 7., 8.], [5., 2., 4.],];

    let b = array![[1., 9., 0.], [5., 4., 1.],];

    println!("stack, axis 0:\n{:?}\n", stack![Axis(0), a, b]);
    println!("stack, axis 1:\n{:?}\n", stack![Axis(1), a, b]);
    println!("stack, axis 2:\n{:?}\n", stack![Axis(2), a, b]);
    println!("concatenate, axis 0:\n{:?}\n", concatenate![Axis(0), a, b]);
    println!("concatenate, axis 1:\n{:?}\n", concatenate![Axis(1), a, b]);
}
