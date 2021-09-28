#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n- 1]
    }
    let mut tree = tree_dist_library::TreeDist::new(n);
    for &(u, v) in &uv {
        tree.add_edge(u - 1, v - 1);
        tree.add_edge(v - 1, u - 1);
    }
    let dist_0 = tree.distance(0).0.iter().map(|e| e.unwrap()).sum::<usize>();
    let subtree = tree.count_subtree_from(0);
    let mut ans = vec![None; n];
    ans[0] = Some(dist_0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &e in &tree.adjacent[v] {
            if ans[e].is_none() {
                let ans_e = ans[v].unwrap() + n - 2 * subtree[e];
                ans[e] = Some(ans_e);
                queue.push_back(e);
            }
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|e| e.unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

pub mod tree_dist_library {
    /// verified by this(https://atcoder.jp/contests/abc218/submissions/25794641)
    pub struct TreeDist {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
    }

    impl TreeDist {
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
        }
    }

    #[cfg(test)]
    mod tests_tree_dist {
        use super::*;

        #[test]
        fn for_count() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }
    }
}
