// 溢出，环绕

#[cfg(test)]
mod tests {
    #[test]
    fn f1() {
        let mut n: u8 = 0;
        assert_eq!(n.wrapping_sub(1), (0 - 1 + 256) as u8);
        assert_eq!(n.wrapping_sub(2), (0 - 2 + 256) as u8);
        assert_eq!(n.wrapping_sub(3), (0 - 3 + 256) as u8);

        n = 255;
        assert_eq!(n.wrapping_add(1), ((255 + 1) & 255) as u8);
        assert_eq!(n.wrapping_add(2), ((255 + 2) & 255) as u8);
        assert_eq!(n.wrapping_add(3), ((255 + 3) & 255) as u8);

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
