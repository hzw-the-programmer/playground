use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub fn test() {
    // test1();
    // test2();
    // test3();
    test4();
    // test5();
}

fn test1() {
    let root = new_tree(
        Foo { i: 1 },
        new_tree(Foo { i: 2 }, None, None),
        new_tree(Foo { i: 3 }, None, None),
    );
    inorder_traversal(root);
}

fn test2() {
    let root = new_tree(
        Foo { i: 1 },
        new_tree(Foo { i: 2 }, None, None),
        new_tree(Foo { i: 3 }, None, None),
    );
    inorder_traversal_preserve(root);
}

fn test3() {
    let root = new_tree(
        Foo { i: 1 },
        new_tree(Foo { i: 2 }, None, None),
        new_tree(Foo { i: 3 }, None, None),
    );
    inorder_traversal_preserve_2(root);
}

fn test4() {
    let root = new_tree(
        Foo { i: 1 },
        new_tree(Foo { i: 2 }, None, None),
        new_tree(Foo { i: 3 }, None, None),
    );
    inorder_traversal_preserve_3(root);
}

fn test5() {
    let root = new_tree(
        Foo { i: 1 },
        new_tree(Foo { i: 2 }, None, None),
        new_tree(Foo { i: 3 }, None, None),
    );
    preorder_traversal(root);
}

#[derive(Debug)]
struct Foo {
    i: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} droped", self.i);
    }
}

type Tree<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Node<T> {
    fn new(val: T, left: Tree<T>, right: Tree<T>) -> Self {
        Self { val, left, right }
    }
}

fn new_tree<T>(val: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
    Some(Rc::new(RefCell::new(Node::new(val, left, right))))
}

fn inorder_traversal<T: Debug>(mut root: Tree<T>) {
    let mut stack = vec![];
    while root.is_some() || !stack.is_empty() {
        while let Some(node) = root {
            println!("1");
            root = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            println!("2");
            println!("{:?}", node.borrow().val);
            assert!(node.borrow().left.is_none());
            root = node.borrow_mut().right.take();
            println!("3");
        }
    }
}

fn inorder_traversal_preserve<T: Debug>(root: Tree<T>) {
    let mut root = root.clone();
    let mut stack = vec![];
    while root.is_some() || !stack.is_empty() {
        while let Some(node) = root {
            println!("1");
            root = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            println!("2");
            println!("{:?}", node.borrow().val);
            assert!(node.borrow().left.is_none());
            root = node.borrow_mut().right.take();
            println!("3");
        }
    }
}

fn inorder_traversal_preserve_2<T: Debug>(root: Tree<T>) {
    let mut root = root.clone();
    let mut stack = vec![];
    while root.is_some() || !stack.is_empty() {
        while let Some(node) = root {
            println!("1");
            root = node.borrow().left.clone();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            println!("2");
            println!("{:?}", node.borrow().val);
            // assert!(node.borrow().left.is_none());
            root = node.borrow_mut().right.take();
            println!("3");
        }
    }
    println!("leave");
}

fn inorder_traversal_preserve_3<T: Debug>(root: Tree<T>) {
    let mut root = root.clone();
    let mut stack = vec![];
    while root.is_some() || !stack.is_empty() {
        while let Some(node) = root {
            println!("1");
            root = node.borrow().left.clone();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            println!("2");
            println!("{:?}", node.borrow().val);
            // assert!(node.borrow().left.is_none());
            root = node.borrow().right.clone();
            println!("3");
        }
    }
    println!("leave");
}

fn preorder_traversal<T: Debug>(mut root: Tree<T>) {
    let mut stack = vec![];
    while root.is_some() || !stack.is_empty() {
        while let Some(node) = root {
            println!("1");
            println!("{:?}", node.borrow().val);
            stack.push(node.borrow_mut().right.take());
            root = node.borrow_mut().left.take();
            println!("2");
        }

        if let Some(tree) = stack.pop() {
            println!("3");
            root = tree;
        }
    }
}
