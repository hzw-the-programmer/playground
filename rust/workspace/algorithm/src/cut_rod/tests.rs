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
