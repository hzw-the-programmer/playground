use super::LinkedList;

#[test]
fn basic_operations() {
    let mut list = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());

    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get(0), Some(&2));
    assert_eq!(list.get(1), Some(&1));
    assert_eq!(list.get(2), Some(&3));

    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_front(), Some(1));
    assert!(list.is_empty());
}

#[test]
fn insert_remove() {
    let mut list = LinkedList::new();
    list.insert(0, 10);
    list.insert(1, 20);
    list.insert(1, 15);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get(0), Some(&10));
    assert_eq!(list.get(1), Some(&15));
    assert_eq!(list.get(2), Some(&20));

    assert_eq!(list.remove(1), Some(15));
    assert_eq!(list.len(), 2);
    assert_eq!(list.remove(0), Some(10));
    assert_eq!(list.remove(0), Some(20));
    assert!(list.is_empty());
}
