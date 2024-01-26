fn main() {
    let mut n = -4_i32;
    println!("{n:b}");
    n = n >> 1;
    println!("{n:b}");
    n = n << 1;
    println!("{n:b}");

    let mut n = 4_u32;
    println!("{n:032b}");
    n = n >> 1;
    println!("{n:032b}");
    n = n << 1;
    println!("{n:032b}");
}
