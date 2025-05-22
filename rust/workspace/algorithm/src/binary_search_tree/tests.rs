use super::*;

#[test]
fn test_basic_operations() {
    let mut bst = BinarySearchTree::new();
    assert!(!bst.contains(&5));

    bst.insert(5);
    assert!(bst.contains(&5));

    bst.insert(3);
    bst.insert(7);
    assert!(bst.contains(&3));
    assert!(bst.contains(&7));

    bst.delete(&5);
    assert!(!bst.contains(&5));
    assert!(bst.contains(&3));
    assert!(bst.contains(&7));
}

#[test]
fn test_delete() {
    let mut bst = BinarySearchTree::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(2);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    assert!(bst.delete(&5));
    assert!(!bst.contains(&5));
    assert!(bst.contains(&3));
    assert!(bst.contains(&7));
    assert!(bst.contains(&6));
    assert!(bst.contains(&8));

    assert!(bst.delete(&7));
    assert!(bst.contains(&6));
    assert!(bst.contains(&8));
    assert!(!bst.contains(&7));
}

#[test]
fn test_in_order() {
    let mut bst = BinarySearchTree::new();

    bst.insert(5);
    bst.insert(4);
    bst.insert(3);
    bst.insert(2);
    bst.insert(1);
    bst.insert(0);

    assert_eq!(
        vec![0, 1, 2, 3, 4, 5].iter().collect::<Vec<_>>(),
        bst.in_order()
    );
}

#[test]
fn test_inorder_iterative() {
    let mut bst = BinarySearchTree::new();

    bst.insert(5);
    bst.insert(4);
    bst.insert(3);
    bst.insert(2);
    bst.insert(1);
    bst.insert(0);

    assert_eq!(
        vec![0, 1, 2, 3, 4, 5].iter().collect::<Vec<_>>(),
        bst.inorder_iterative()
    );
}

#[test]
fn test_iter() {
    let mut bst = BinarySearchTree::new();

    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(2);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    let mut i = bst.iter();
    assert_eq!(Some(&2), i.next());
    assert_eq!(Some(&3), i.next());
    assert_eq!(Some(&4), i.next());
    assert_eq!(Some(&5), i.next());
    assert_eq!(Some(&6), i.next());
    assert_eq!(Some(&7), i.next());
    assert_eq!(Some(&8), i.next());
    assert_eq!(None, i.next());
}
