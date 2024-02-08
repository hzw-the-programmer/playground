use ndarray::prelude::*;
use ndarray::Array;

fn main() {
    let a = Array::range(0., 10., 1.);
    println!("{}", a);

    let mut a = a.mapv(|a: f64| a.powi(3)); // numpy equivlant of `a ** 3`; https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.powi

    println!("{}", a);

    println!("{}", a[[2]]);
    println!("{}", a.slice(s![2]));

    println!("{}", a.slice(s![2..5]));

    a.slice_mut(s![..6;2]).fill(1000.); // numpy equivlant of `a[:6:2] = 1000`
    println!("{}", a);

    for i in a.iter() {
        print!("{}, ", i.powf(1. / 3.))
    }
}
