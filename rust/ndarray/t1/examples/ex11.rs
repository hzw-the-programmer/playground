use ndarray::prelude::*;
use ndarray::Axis;

fn main() {
    let a = array![
        [6., 7., 6., 9., 0., 5., 4., 0., 6., 8., 5., 2.],
        [8., 5., 5., 7., 1., 8., 6., 7., 1., 8., 1., 0.]
    ];
    println!("{}\n", a);

    let (s1, s2) = a.view().split_at(Axis(0), 1);
    println!("Split a from Axis(0), at index 1:");
    println!("s1  = \n{}", s1);
    println!("s2  = \n{}\n", s2);

    let (s1, s2) = a.view().split_at(Axis(1), 4);
    println!("Split a from Axis(1), at index 4:");
    println!("s1  = \n{}", s1);
    println!("s2  = \n{}\n", s2);
}
