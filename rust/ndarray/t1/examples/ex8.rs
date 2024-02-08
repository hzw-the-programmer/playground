use ndarray::prelude::*;

fn main() {
    let a = array![
        [
            [0, 1, 2], // a 3D array  2 x 2 x 3
            [10, 12, 13]
        ],
        [[100, 101, 102], [110, 112, 113]]
    ];
    println!("{:?}", a);

    let a = a.mapv(|a: isize| a.pow(1)); // numpy equivalent of `a ** 1`;
                                         // This line does nothing except illustrating mapv with isize type
    println!("a -> \n{}\n", a);

    println!("`a.slice(s![1, .., ..])` -> \n{}\n", a.slice(s![1, .., ..]));

    println!("`a.slice(s![.., .., 2])` -> \n{}\n", a.slice(s![.., .., 2]));

    println!(
        "`a.slice(s![.., 1, 0..2])` -> \n{}\n",
        a.slice(s![.., 1, 0..2])
    );

    println!("`a.iter()` ->");
    for i in a.iter() {
        print!("{}, ", i) // flat out to every element
    }

    println!("\n\n`a.outer_iter()` ->");
    for i in a.outer_iter() {
        print!("row: {}, \n", i) // iterate through first dimension
    }
}
