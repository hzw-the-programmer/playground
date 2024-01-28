pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn print_rand() {
    println!("{}", rand::random::<u8>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
