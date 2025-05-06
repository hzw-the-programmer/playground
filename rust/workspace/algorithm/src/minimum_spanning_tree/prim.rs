use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn prim(graph: &[Vec<(usize, u32)>]) -> u32 {
    let mut visited = vec![false; graph.len()];
    let mut weight = 0;
    let mut pq = BinaryHeap::new();

    pq.push(Reverse((0, 0)));

    while let Some(Reverse((w, v))) = pq.pop() {
        if visited[v] {
            continue;
        }

        visited[v] = true;
        weight += w;

        for &(v, w) in &graph[v] {
            if !visited[v] {
                pq.push(Reverse((w, v)));
            }
        }
    }

    weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prim() {
        let graph = vec![
            vec![(1, 2), (3, 6)],
            vec![(0, 2), (2, 3), (3, 8), (4, 5)],
            vec![(1, 3), (4, 7)],
            vec![(0, 6), (1, 8)],
            vec![(1, 5), (2, 7)],
        ];
        assert_eq!(16, prim(&graph));
    }
}
