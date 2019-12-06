use std::collections::{HashMap, HashSet, VecDeque};

/// A Compressed Spase Row graph.

#[derive(Debug)]
pub struct CsrGraph {
    weights: Vec<usize>,
    row_count: Vec<usize>,
    columns: Vec<usize>,
}

impl CsrGraph {
    pub fn new() -> CsrGraph {
        CsrGraph {
            weights: Vec::new(),
            row_count: vec![0],
            columns: Vec::new(),
        }
    }

    pub fn with_nodes(n: usize) -> CsrGraph {
        CsrGraph {
            weights: Vec::new(),
            row_count: vec![0; n + 1],
            columns: Vec::new(),
        }
    }

    pub fn from_adj_matrix(mat: Vec<Vec<usize>>) -> CsrGraph {
        assert!(
            mat.iter().all(|row| row.len() == mat.len()),
            "Not a square matrix"
        );
        let mut weights = Vec::new();
        let mut row_count = vec![0];
        let mut columns = Vec::new();
        let mut non_zero_cnt = 0;

        for row in mat {
            for (col, &item) in row.iter().enumerate().filter(|(_, &i)| i != 0) {
                weights.push(item);
                columns.push(col);
                non_zero_cnt += 1;
            }
            row_count.push(non_zero_cnt);
        }

        CsrGraph {
            weights,
            row_count,
            columns,
        }
    }

    pub fn node_count(&self) -> usize {
        assert!(!self.row_count.is_empty());
        self.row_count.len() - 1
    }

    pub fn add_node(&mut self) {
        if let Some(&last) = self.row_count.last() {
            self.row_count.push(last);
        } else {
            unreachable!("row_count is empty");
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize) -> bool {
        assert!(
            from < self.node_count() && to < self.node_count(),
            "Invalid node"
        );
        let (from, to) = if from > to { (to, from) } else { (from, to) };
        let mut place = self.row_count[from];
        let end = self.row_count[from + 1];

        while place < end && self.columns[place] < to {
            place += 1;
        }

        if place < end && to == self.columns[place] {
            // Edge already exists
            false
        } else {
            self.weights.insert(place, weight);
            self.columns.insert(place, to);
            self.row_count[from + 1..].iter_mut().for_each(|n| *n += 1);
            true
        }
    }

    /// Creates an iterator over adjacent nodes of `node`.
    pub fn adjs(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        let start = self.row_count[node];
        let end = self.row_count[node + 1];
        self.rev_adjs(node).chain(
            self.columns[start..end]
                .iter()
                .cloned()
                .filter(move |&n| n > node),
        )
    }

    fn rev_adjs(&self, node: usize) -> RevAdjIter {
        RevAdjIter {
            graph: self,
            to_node: node,
            curr_on_col: 0,
            curr_on_row: 0,
        }
    }

    pub fn bfs<F>(&self, start: usize, mut visitor: F)
    where
        F: FnMut(usize),
    {
        let mut deque = VecDeque::new();
        let mut visited = HashSet::new();
        deque.push_back(start);
        visited.insert(start);
        while let Some(node) = deque.pop_front() {
            visitor(node);
            for n in self.adjs(node) {
                if !visited.contains(&n) {
                    visited.insert(n);
                    deque.push_back(n);
                }
            }
        }
    }

    /// Finds a path from `src` to `dst` using bidirectional BFS.
    pub fn bidir_bfs(&self, src: usize, dst: usize) -> Vec<usize> {
        assert!(
            src < self.node_count() && dst < self.node_count(),
            "Invalid node"
        );
        let mut src_q = VecDeque::new();
        let mut dst_q = VecDeque::new();
        let mut src_prt = HashMap::new();
        let mut dst_prt = HashMap::new();

        src_q.push_back(src);
        dst_q.push_back(dst);

        let intersect = loop {
            match (src_q.pop_front(), dst_q.pop_front()) {
                (Some(sn), Some(dn)) => {
                    if sn == dst || dst_prt.contains_key(&sn) {
                        break Some(sn);
                    }
                    for n in self.adjs(sn) {
                        if n != src && !src_prt.contains_key(&n) {
                            src_prt.insert(n, sn);
                            src_q.push_back(n);
                        }
                    }

                    if dn == sn || src_prt.contains_key(&dn) {
                        break Some(dn);
                    }
                    for n in self.adjs(dn) {
                        if n != dst && !dst_prt.contains_key(&n) {
                            dst_prt.insert(n, dn);
                            dst_q.push_back(n);
                        }
                    }
                }
                _ => break None,
            }
        };

        let mut ret = Vec::new();
        if let Some(node) = intersect {
            let mut n = node;
            while let Some(&pa) = src_prt.get(&n) {
                ret.insert(0, pa);
                n = pa;
            }
            ret.push(node);
            let mut n = node;
            while let Some(&pa) = dst_prt.get(&n) {
                ret.push(pa);
                n = pa;
            }
        }
        ret
    }
}

struct RevAdjIter<'a> {
    graph: &'a CsrGraph,
    to_node: usize,
    curr_on_col: usize,
    curr_on_row: usize,
}

impl<'a> Iterator for RevAdjIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cstart = self.curr_on_col;
        for (&c, i) in self.graph.columns[cstart..].iter().zip(cstart..) {
            if c != self.to_node {
                continue;
            }
            let rstart = self.curr_on_row;
            for (&r, j) in self.graph.row_count[rstart..].iter().zip(rstart..) {
                if r > i {
                    self.curr_on_col = i + 1;
                    self.curr_on_row = j + 1;
                    return Some(j - 1);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node() {
        let mut g = CsrGraph::with_nodes(10);
        assert_eq!(g.node_count(), 10);
        g.add_edge(0, 9, 3);
        g.add_edge(9, 8, 4);
    }

    #[test]
    fn add_edge() {
        let mat = vec![
            vec![0, 5, 3, 0],
            vec![0, 0, 0, 6],
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
        ];
        let mut g = CsrGraph::from_adj_matrix(mat);
        assert!(g.add_edge(2, 1, 5));
        assert_eq!(g.weights, vec![5, 3, 5, 6, 3]);
        assert_eq!(g.row_count, vec![0, 2, 4, 5, 5]);
        assert_eq!(g.columns, vec![1, 2, 2, 3, 2]);

        assert!(g.add_edge(2, 3, 5));
        assert_eq!(g.weights, vec![5, 3, 5, 6, 3, 5]);
        assert_eq!(g.row_count, vec![0, 2, 4, 6, 6]);
        assert_eq!(g.columns, vec![1, 2, 2, 3, 2, 3]);

        assert!(!g.add_edge(1, 2, 1));
        assert!(!g.add_edge(2, 2, 1));

        assert!(g.add_edge(0, 0, 1));
        assert_eq!(g.weights, vec![1, 5, 3, 5, 6, 3, 5]);
        assert_eq!(g.row_count, vec![0, 3, 5, 7, 7]);
        assert_eq!(g.columns, vec![0, 1, 2, 2, 3, 2, 3]);

        assert!(g.add_edge(0, 3, 2));
        assert_eq!(g.weights, vec![1, 5, 3, 2, 5, 6, 3, 5]);
        assert_eq!(g.row_count, vec![0, 4, 6, 8, 8]);
        assert_eq!(g.columns, vec![0, 1, 2, 3, 2, 3, 2, 3]);
    }

    #[test]
    fn adjs() {
        let mat = vec![
            vec![0, 5, 3, 0],
            vec![0, 0, 2, 8],
            vec![0, 0, 3, 1],
            vec![0, 0, 0, 0],
        ];
        let g = CsrGraph::from_adj_matrix(mat);
        assert_eq!(g.adjs(1).collect::<Vec<_>>(), vec![0, 2, 3]);
        assert_eq!(g.adjs(2).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn rev_adjs() {
        let mat = vec![
            vec![0, 0, 3, 0],
            vec![5, 0, 2, 8],
            vec![0, 0, 3, 0],
            vec![0, 6, 0, 0],
        ];
        let g = CsrGraph::from_adj_matrix(mat);

        assert_eq!(g.rev_adjs(0).collect::<Vec<_>>(), vec![1]);
        assert_eq!(g.rev_adjs(1).collect::<Vec<_>>(), vec![3]);
        assert_eq!(g.rev_adjs(2).collect::<Vec<_>>(), vec![0, 1, 2]);
        assert_eq!(g.rev_adjs(3).collect::<Vec<_>>(), vec![1]);

        let mat = vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 2, 8],
            vec![0, 0, 3, 0],
            vec![0, 6, 1, 0],
        ];
        let g = CsrGraph::from_adj_matrix(mat);
        assert_eq!(g.rev_adjs(0).collect::<Vec<_>>(), vec![]);
        assert_eq!(g.rev_adjs(2).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn bidir_bfs() {
        let mut g = CsrGraph::with_nodes(15);
        let edges = [
            (0, 4),
            (1, 4),
            (2, 5),
            (3, 5),
            (4, 6),
            (5, 6),
            (7, 8),
            (8, 9),
            (8, 10),
            (9, 11),
            (9, 12),
            (10, 13),
            (10, 14),
        ];
        for &(from, to) in edges.iter() {
            g.add_edge(from, to, 1);
        }
        assert_eq!(g.bidir_bfs(0, 14), vec![]);

        g.add_edge(6, 7, 1);
        assert_eq!(g.bidir_bfs(0, 14), vec![0, 4, 6, 7, 8, 10, 14]);
        assert_eq!(g.bidir_bfs(0, 2), vec![0, 4, 6, 5, 2]);
        assert_eq!(g.bidir_bfs(3, 2), vec![3, 5, 2]);
        assert_eq!(g.bidir_bfs(14, 0), vec![14, 10, 8, 7, 6, 4, 0]);
        assert_eq!(g.bidir_bfs(6, 8), vec![6, 7, 8]);
        assert_eq!(g.bidir_bfs(5, 5), vec![5]);
    }
}
