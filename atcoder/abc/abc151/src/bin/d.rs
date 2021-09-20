#![allow(unreachable_code)]
#![allow(dead_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [String; h]
    }
    let ss = ss
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let mut tree = tree_dist_library::TreeDist::new(h * w);
    let dx: [isize; 4] = [-1, 0, 1, 0];
    let dy: [isize; 4] = [0, -1, 0, 1];
    for i in 0..w {
        for j in 0..h {
            if ss[j as usize * w + i as usize] == '#' {
                continue;
            }
            for (x, y) in dx.iter().zip(dy.iter()) {
                let new_x = i as isize + x;
                let new_y = j as isize + y;
                if 0 <= new_x
                    && new_x < w as isize
                    && 0 <= new_y
                    && new_y < h as isize
                    && ss[new_y as usize * w + new_x as usize] == '.'
                {
                    tree.add_edge(
                        j as usize * w + i as usize,
                        new_y as usize * w + new_x as usize,
                    );
                }
            }
        }
    }
    let mut best = 0;
    for i in 0..h * w {
        best = std::cmp::max(tree.distance(i).0.iter().max().unwrap().unwrap(), best);
    }
    println!("{}", best)
}

pub mod tree_dist_library {
    /// verified by this(https://atcoder.jp/contests/abc218/submissions/25794641)
    pub struct TreeDist {
        nodes: usize,
        adjacent: Vec<Vec<usize>>,
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
    }
}
