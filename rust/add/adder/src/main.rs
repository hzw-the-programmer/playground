fn main() {
    println!("{}", add_one::add_one(10));
    add_one::print_rand();
    println!("{}", rand::random::<u8>());
}
