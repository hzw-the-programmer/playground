pub fn test() {
    test1();
}

fn test1() {
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    println!("{:?}", s.binary_search(&1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search() {
        let s = [1, 3, 5, 7, 9];
        assert_eq!(s.binary_search(&5), Ok(2));
        assert_eq!(s.binary_search(&4), Err(2));
        assert_eq!(s.binary_search(&0), Err(0));
        assert_eq!(s.binary_search(&10), Err(5));
    }
}
