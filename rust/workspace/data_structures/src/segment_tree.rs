pub struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut st = SegmentTree {
            tree: vec![0; 4 * n],
            n,
        };
        st.build(0, 0, n - 1, arr);
        st
    }

    fn build(&mut self, node: usize, start: usize, end: usize, arr: &[i32]) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            let left = 2 * node + 1;
            let right = 2 * node + 2;
            self.build(left, start, mid, arr);
            self.build(right, mid + 1, end, arr);
            self.tree[node] = self.tree[left] + self.tree[right];
        }
    }

    pub fn query(&self, l: usize, r: usize) -> i32 {
        self.query_helper(0, 0, self.n - 1, l, r)
    }

    fn query_helper(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || l > end {
            0
        } else if l <= start && r >= end {
            self.tree[node]
        } else {
            let mid = (start + end) / 2;
            let left = 2 * node + 1;
            let right = 2 * node + 2;
            let left_sum = self.query_helper(left, start, mid, l, r);
            let right_sum = self.query_helper(right, mid + 1, end, l, r);
            left_sum + right_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_query() {
        let arr = [1, 3, 5, 7, 9];
        let st = SegmentTree::new(&arr);
        assert_eq!(st.query(0, 4), 1 + 3 + 5 + 7 + 9);
        assert_eq!(st.query(1, 3), 3 + 5 + 7);
        assert_eq!(st.query(2, 2), 5);
    }
}
