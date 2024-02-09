use std::ops::ControlFlow;

fn main() {
    t1();
    t2();
    t3();
}

fn t1() {
    println!("\nt1");
    let node = TreeNode::leaf(1).unwrap();
    node.traverse_inorder(&mut |v| {
        println!("{v}");
        if *v > 0 {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    });
}

fn t2() {
    println!("\nt2");
    let node = TreeNode::leaf(-1).unwrap();
    let r = node.traverse_inorder(&mut |v| {
        println!("{v}");
        if *v > 0 {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(*v)
        }
    });
    assert_eq!(r, ControlFlow::Break(-1));
}

fn t3() {
    println!("\nt3");
    let node = TreeNode {
        value: 0,
        left: TreeNode::leaf(1),
        right: Some(Box::new(TreeNode {
            value: -1,
            left: TreeNode::leaf(5),
            right: TreeNode::leaf(2),
        })),
    };
    let mut sum = 0;

    let res = node.traverse_inorder(&mut |val| {
        if *val < 0 {
            ControlFlow::Break(*val)
        } else {
            sum += *val;
            ControlFlow::Continue(())
        }
    });
    assert_eq!(res, ControlFlow::Break(-1));
    assert_eq!(sum, 6);
}

pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: std::fmt::Display + Copy> TreeNode<T> {
    pub fn traverse_inorder<B>(&self, f: &mut impl FnMut(&T) -> ControlFlow<B>) -> ControlFlow<B> {
        let v = self.value;

        if let Some(left) = &self.left {
            println!("before left: {v}");
            left.traverse_inorder(f)?;
            println!("after left: {v}");
        }

        println!("before self: {v}");
        f(&self.value)?;
        println!("after self: {v}");

        if let Some(right) = &self.right {
            println!("before right: {v}");
            right.traverse_inorder(f)?;
            println!("after right: {v}");
        }

        ControlFlow::Continue(())
    }

    fn leaf(value: T) -> Option<Box<TreeNode<T>>> {
        Some(Box::new(Self {
            value,
            left: None,
            right: None,
        }))
    }
}
