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

    pub fn update(&mut self, index: usize, value: i32) {
        self.update_helper(0, 0, self.n - 1, index, value);
    }

    fn update_helper(&mut self, node: usize, start: usize, end: usize, index: usize, value: i32) {
        if start == end {
            self.tree[node] = value;
        } else {
            let mid = (start + end) / 2;
            let left = 2 * node + 1;
            let right = 2 * node + 2;
            if index <= mid {
                self.update_helper(left, start, mid, index, value);
            } else {
                self.update_helper(right, mid + 1, end, index, value);
            }
            self.tree[node] = self.tree[left] + self.tree[right];
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

    #[test]
    fn test_update() {
        let mut st = SegmentTree::new(&[1, 2, 3, 4]);
        assert_eq!(st.query(0, 3), 1 + 2 + 3 + 4);

        st.update(1, 10);
        assert_eq!(st.query(0, 3), 1 + 10 + 3 + 4);

        st.update(3, 5);
        assert_eq!(st.query(2, 3), 3 + 5);
    }
}
