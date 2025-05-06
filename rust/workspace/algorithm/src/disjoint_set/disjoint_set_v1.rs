#[derive(Debug)]
pub struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect::<Vec<_>>(),
        }
    }

    pub fn find(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.find(self.parent[x])
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        self.parent[root_y] = root_x;
        // self.parent[root_x] = root_y;
    }

    pub fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
