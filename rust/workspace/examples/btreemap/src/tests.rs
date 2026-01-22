use core::ops::Bound;
use std::collections::BTreeMap;

#[test]
fn lower_bound() {
    let mut m = BTreeMap::new();
    m.insert(2, 4);
    m.insert(7, 10);

    let c = m.lower_bound(Bound::Unbounded);
    assert_eq!(c.peek_prev(), None);
    assert_eq!(c.peek_next(), Some((&2, &4)));
}

#[test]
fn upper_bound() {
    let mut m = BTreeMap::from([(2, 4), (7, 10)]);

    let c = m.upper_bound(Bound::Unbounded);
    assert_eq!(c.peek_prev(), Some((&7, &10)));
    assert_eq!(c.peek_next(), None);

    let c = m.upper_bound(Bound::Included(&5));
    assert_eq!(c.peek_prev(), Some((&2, &4)));
    assert_eq!(c.peek_next(), Some((&7, &10)));
}
