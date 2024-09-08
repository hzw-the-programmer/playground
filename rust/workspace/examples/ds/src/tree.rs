pub fn test() {
    let mut root = Tree::new();
    root.add(10);
    root.add(8);
    root.add(9);
    root.add(7);
    root.add(11);
    root.print();
}

struct Tree<T>(Option<Box<Node<T>>>);

struct Node<T> {
    left: Tree<T>,
    right: Tree<T>,
    data: T,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Tree(None)
    }
}

impl<T: PartialOrd + std::fmt::Debug> Tree<T> {
    fn add(&mut self, data: T) {
        match self.0 {
            None => {
                self.0.replace(Box::new(Node::new(data)));
            }
            Some(ref mut t) => {
                if data < t.data {
                    t.left.add(data);
                } else {
                    t.right.add(data);
                }
            }
        }
    }

    fn print(&self) {
        match self.0 {
            None => {}
            Some(ref t) => {
                t.left.print();
                println!("{:?}", t.data);
                t.right.print();
            }
        }
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            left: Tree::new(),
            right: Tree::new(),
            data,
        }
    }
}
