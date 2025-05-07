use super::cut_rod;

#[test]
fn test_zero_length() {
    let prices = &[];
    assert_eq!(cut_rod(prices, 0), 0);
}

#[test]
fn test_length_one() {
    let prices = &[1];
    assert_eq!(cut_rod(prices, 1), 1);
}

#[test]
fn test_length_four() {
    let prices = &[1, 5, 8, 9];
    assert_eq!(cut_rod(prices, 4), 10);
}

#[test]
fn test_longer_than_prices() {
    let prices = &[1, 5, 8, 9];
    assert_eq!(cut_rod(prices, 5), 13);
}

#[test]
fn test_optimal_cut() {
    let prices = &[1, 5, 8, 9, 10, 17, 17, 20];
    assert_eq!(cut_rod(prices, 8), 22);
}

#[test]
fn test_base_cases() {
    // 长度为 0 的情况
    assert_eq!(cut_rod(&[], 0), 0);
    // 长度为 1 且价格合理
    assert_eq!(cut_rod(&[3], 1), 3);
    // 价格数组为空但长度非零
    assert_eq!(cut_rod(&[], 5), 0);
}

#[test]
fn test_standard_example() {
    let prices = vec![1, 5, 8, 9];
    // 经典问题验证
    assert_eq!(cut_rod(&prices, 4), 10); // 最佳切割：2+2
    assert_eq!(cut_rod(&prices, 5), 13); // 最佳切割：2+3
    assert_eq!(cut_rod(&prices, 3), 8); // 不切割
}

#[test]
fn test_edge_cases() {
    // 所有切割价格相同
    let uniform_prices = vec![1, 1, 1, 1];
    assert_eq!(cut_rod(&uniform_prices, 4), 4); // 四段 1

    // 最佳方案在最后一段
    let peak_end_prices = vec![0, 0, 0, 10];
    assert_eq!(cut_rod(&peak_end_prices, 4), 10); // 直接取最后一段

    // 长度超过价格数组长度
    let short_prices = vec![1, 5];
    assert_eq!(cut_rod(&short_prices, 3), 6); // 1+1+1 vs 5+1
}

#[test]
fn test_cut_rod() {
    let tests = [
        (&[1, 5, 8, 9, 10, 17, 17, 20][..], 22),
        (&[3, 5, 8, 9, 10, 17, 17, 20][..], 24),
        (&[3][..], 3),
    ];
    for (i, test) in tests.iter().enumerate() {
        assert_eq!(cut_rod(test.0, test.0.len()), test.1, "{i} failed");
    }
}
