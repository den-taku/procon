#![allow(unreachable_code)]
#![allow(dead_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    }
    let mut tree = tree_dist_library::TreeDist::new(n);
    for &(a, b) in &edges {
        tree.add_edge(a - 1, b - 1);
    }
    let (dist, pre) = tree.distance(0);
    if dist[n - 1].is_none() {
        println!("{}", vec!["-1".to_string(); m].join("\n"))
    } else {
        let mut set = std::collections::HashSet::new();
        let mut u = pre[n - 1];
        let mut v = n - 1;
        set.insert((u, v));
        while v != 0 {
            v = u;
            u = pre[u];
            set.insert((u, v));
        }
        let mut ans = Vec::new();
        for &(a, b) in &edges {
            if set.contains(&(a - 1, b - 1)) {
                tree.remove_edge(a - 1, b - 1);
                let (dist, _) = tree.distance(0);
                tree.add_edge(a - 1, b - 1);
                if let Some(d) = dist[n - 1] {
                    ans.push(d.to_string())
                } else {
                    ans.push("-1".to_string())
                }
            } else {
                ans.push(dist[n - 1].unwrap().to_string())
            }
        }
        println!("{}", ans.join("\n"))
    }
}

pub mod tree_dist_library {
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
