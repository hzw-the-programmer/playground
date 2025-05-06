use std::cmp::Ordering;

struct Edge {
    u: usize,
    v: usize,
    w: u32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        other.w == self.w
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.w.partial_cmp(&other.w)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w)
    }
}

pub fn kruskal(graph: &[Vec<(usize, u32)>]) -> u32 {
    let n = graph.len();

    let mut edges = Vec::new();
    for u in 0..n {
        for &(v, w) in &graph[u] {
            if u < v {
                edges.push(Edge { u, v, w });
            }
        }
    }
    edges.sort();

    let mut min_weight = 0;

    let mut ds = crate::disjoint_set::DisjointSet::new(n);
    for edge in edges {
        let root_u = ds.find(edge.u);
        let root_v = ds.find(edge.v);
        if root_u != root_v {
            min_weight += edge.w;
            ds.union(root_u, root_v);
        }
    }

    min_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let graph = vec![
            vec![(1, 2), (3, 6)],
            vec![(0, 2), (2, 3), (3, 8), (4, 5)],
            vec![(1, 3), (4, 7)],
            vec![(0, 6), (1, 8)],
            vec![(1, 5), (2, 7)],
        ];
        assert_eq!(16, kruskal(&graph));
    }
}
