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
