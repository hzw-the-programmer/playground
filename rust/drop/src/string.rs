#[cfg(test)]
mod tests {
    #[test]
    fn as_mut_str() {
        let mut s = String::from("hello world!");
        let s1 = s.as_mut_str();
        s1.make_ascii_uppercase();
        assert_eq!("HELLO WORLD!", s);
        // assert_eq!("HELLO WORLD!", s1);
    }
}
