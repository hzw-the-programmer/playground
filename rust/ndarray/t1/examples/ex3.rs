use ndarray::Array;
fn main() {
    let a = Array::<f64, _>::linspace(0., 5., 11);
    println!("{:?}", a);
}
