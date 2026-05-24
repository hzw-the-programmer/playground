/// 溢出
/// Debug panic
/// Release 环绕
/// cargo test
/// cargo test --release

#[cfg(test)]
mod tests {

    /// Debug模式加法溢出触发panic
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "attempt to add with overflow")]
    #[allow(arithmetic_overflow)]
    fn add_overflow_debug_panic() {
        let max = u8::MAX;
        let _ = max + 1;
    }

    /// Release模式加法溢出二进制环绕
    #[test]
    #[cfg(not(debug_assertions))]
    #[allow(arithmetic_overflow)]
    fn add_overflow_release_wrapping() {
        let max = u8::MAX;
        let res = max + 1;
        assert_eq!(res, 0);
    }
}
