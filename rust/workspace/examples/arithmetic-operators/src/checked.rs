// 溢出，返回 None

#[cfg(test)]
mod tests {
    #[test]
    fn f1() {
        let mut n: u8 = 0;
        assert_eq!(n.checked_sub(1), None);
        assert_eq!(n.checked_sub(2), None);
        assert_eq!(n.checked_sub(3), None);

        n = 255;
        assert_eq!(n.checked_add(1), None);
        assert_eq!(n.checked_add(2), None);
        assert_eq!(n.checked_add(3), None);

        n = 20;
        assert_eq!(n.checked_mul(10), Some(200));

        let mut n: i8 = -128;
        assert_eq!(n.checked_sub(1), None);
        assert_eq!(n.checked_sub(2), None);
        assert_eq!(n.checked_sub(3), None);

        n = 127;
        assert_eq!(n.checked_add(1), None);
        assert_eq!(n.checked_add(2), None);
        assert_eq!(n.checked_add(3), None);
    }
}
