// 溢出，环绕

#[cfg(test)]
mod tests {
    #[test]
    fn f1() {
        let mut n: u8 = 0;
        assert_eq!(n.wrapping_sub(1), 255);
        assert_eq!(n.wrapping_sub(2), 254);
        assert_eq!(n.wrapping_sub(3), 253);

        n = 255;
        assert_eq!(n.wrapping_add(1), 0);
        assert_eq!(n.wrapping_add(2), 1);
        assert_eq!(n.wrapping_add(3), 2);

        let mut n: i8 = -128;
        assert_eq!(n.wrapping_sub(1), 127);
        assert_eq!(n.wrapping_sub(2), 126);
        assert_eq!(n.wrapping_sub(3), 125);

        n = 127;
        assert_eq!(n.wrapping_add(1), -128);
        assert_eq!(n.wrapping_add(2), -127);
        assert_eq!(n.wrapping_add(3), -126);
    }
}
