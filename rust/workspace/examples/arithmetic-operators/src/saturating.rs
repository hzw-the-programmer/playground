// 溢出，返回类型最大或最小值

#[cfg(test)]
mod tests {
    #[test]
    fn f1() {
        let mut n: u8 = 0;
        assert_eq!(n.saturating_sub(1), 0);
        assert_eq!(n.saturating_sub(2), 0);
        assert_eq!(n.saturating_sub(3), 0);

        n = 255;
        assert_eq!(n.saturating_add(1), 255);
        assert_eq!(n.saturating_add(2), 255);
        assert_eq!(n.saturating_add(3), 255);

        let mut n: i8 = -128;
        assert_eq!(n.saturating_sub(1), -128);
        assert_eq!(n.saturating_sub(2), -128);
        assert_eq!(n.saturating_sub(3), -128);

        n = 127;
        assert_eq!(n.saturating_add(1), 127);
        assert_eq!(n.saturating_add(2), 127);
        assert_eq!(n.saturating_add(3), 127);
    }
}
