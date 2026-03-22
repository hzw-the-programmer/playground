use super::SafeLinkedList;

#[test]
fn t1() {
    let mut list = SafeLinkedList::new();

    assert!(list.is_empty());

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.len(), 3);

    assert!(list.remove(&2));
    assert_eq!(list.len(), 2);

    list.push_back(4);

    let mut iter = list.iter();
    assert_eq!(iter.next().unwrap().borrow().value, 1);
    assert_eq!(iter.next().unwrap().borrow().value, 3);
    assert_eq!(iter.next().unwrap().borrow().value, 4);
    assert!(iter.next().is_none());

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert!(iter.next().is_none());
}
