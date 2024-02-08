use ndarray::prelude::*;
use ndarray::{Array, Axis};

fn main() {
    let mut a = Array::range(0., 12., 1.).into_shape([3, 4]).unwrap();
    println!("a = \n{}\n", a);

    {
        let (s1, s2) = a.view().split_at(Axis(1), 2);

        // with s as a view sharing the ref of a, we cannot update a here
        // a.slice_mut(s![1, 1]).fill(1234.);

        println!("Split a from Axis(1), at index 2:");
        println!("s1  = \n{}", s1);
        println!("s2  = \n{}\n", s2);
    }

    // now we can update a again here, as views of s1, s2 are dropped already
    a.slice_mut(s![1, 1]).fill(1234.);

    let (s1, s2) = a.view().split_at(Axis(1), 2);
    println!("Split a from Axis(1), at index 2:");
    println!("s1  = \n{}", s1);
    println!("s2  = \n{}\n", s2);
}
