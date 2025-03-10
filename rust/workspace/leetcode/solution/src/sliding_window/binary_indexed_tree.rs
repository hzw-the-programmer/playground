pub struct BinaryIndexedTree(Vec<i32>);

impl BinaryIndexedTree {
    pub fn new(len: usize) -> Self {
        BinaryIndexedTree(vec![0; len + 1])
    }

    pub fn update(&mut self, i: usize, d: i32) {
        let mut i = i + 1;
        while i < self.0.len() {
            self.0[i] += d;
            i += i & i.wrapping_neg();
        }
    }

    pub fn query(&self, i: usize) -> i32 {
        let mut i = i + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.0[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }

    pub fn range(&self, s: usize, e: usize) -> i32 {
        if s == 0 {
            self.query(e)
        } else {
            self.query(e) - self.query(s - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut tree = BinaryIndexedTree::new(a.len());
        for (i, &e) in a.iter().enumerate() {
            tree.update(i, e);
        }
        assert_eq!(1, tree.query(0));
        assert_eq!(1 + 2, tree.query(1));
        assert_eq!(1 + 2 + 3, tree.query(2));
        assert_eq!(1 + 2 + 3 + 4, tree.query(3));
        assert_eq!(1 + 2 + 3 + 4 + 5, tree.query(4));
        assert_eq!(1 + 2 + 3 + 4 + 5 + 6, tree.query(5));
        assert_eq!(1 + 2 + 3 + 4 + 5 + 6 + 7, tree.query(6));
        assert_eq!(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8, tree.query(7));

        assert_eq!(1 + 2 + 3 + 4 + 5, tree.range(0, 4));
        assert_eq!(5 + 6 + 7 + 8, tree.range(4, 7));
    }
}
