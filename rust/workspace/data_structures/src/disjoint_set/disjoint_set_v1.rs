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
        let xrep = self.find(x);
        let yrep = self.find(y);
        self.parent[xrep] = yrep;
    }

    pub fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
