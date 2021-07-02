#![allow(dead_code)]
use dinic_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        g: usize,
        e: usize,
        p: [usize; g],
        f: [(usize, usize); e]
    }
    let mut f: Vec<(usize, usize, i64)> = f
        .iter()
        .map(|(s, t)| vec![(*s, *t, 1i64), (*t, *s, 1)])
        .flatten()
        .collect();
    for &girl in &p {
        f.push((girl, n, 1));
    }
    let mut flow = Dinic::new(n + 1, &f);
    println!("{}", flow.max_flow(0, n));
}

pub mod dinic_library {
    /// edges is expressed as adjacency list.
    #[derive(Clone, Debug)]
    pub struct Dinic<T> {
        edges: Vec<Vec<Edge<T>>>,
        edges_size: usize,
    }

    #[derive(Copy, Clone, Debug)]
    struct Edge<T> {
        to: usize,
        capacity: T,
        rev: usize,
    }

    impl<T> Dinic<T>
    where
        T: std::convert::From<i32> + Clone + Copy,
    {
        fn add_edge(&mut self, from: usize, to: usize, capacity: T) {
            unsafe {
                let rev = self.edges.get_unchecked(to).len();
                self.edges
                    .get_unchecked_mut(from)
                    .push(Edge { to, capacity, rev });
                let rev = self.edges.get_unchecked(from).len() - 1;
                self.edges.get_unchecked_mut(to).push(Edge {
                    to: from,
                    capacity: T::from(0i32),
                    rev,
                });
            }
        }

        pub fn new(n: usize, edges: &[(usize, usize, T)]) -> Self {
            let mut container = Self {
                edges: vec![vec![]; n],
                edges_size: edges.len(),
            };
            for &(from, to, capacity) in edges {
                container.add_edge(from, to, capacity);
            }
            container
        }
    }

    impl<T> Dinic<T>
    where
        T: std::convert::From<i32>
            + std::cmp::Ord
            + Copy
            + std::ops::SubAssign
            + std::ops::AddAssign
            + Max,
    {
        fn bfs(&self, start: usize) -> Vec<i64> {
            unsafe {
                let mut level = vec![-1; self.edges.len()];
                let mut queue = std::collections::VecDeque::with_capacity(self.edges_size);
                *level.get_unchecked_mut(start) = 0;
                queue.push_back(start);
                while !queue.is_empty() {
                    let vertex = queue.pop_front().unwrap();
                    (0..self.edges.get_unchecked(vertex).len()).fold((), |_, i| {
                        let edge = self.edges.get_unchecked(vertex).get_unchecked(i);
                        if edge.capacity > T::from(0) && *level.get_unchecked(edge.to) < 0 {
                            *level.get_unchecked_mut(edge.to) = *level.get_unchecked(vertex) + 1;
                            queue.push_back(edge.to);
                        }
                    });
                }
                level
            }
        }

        fn dfs(
            &mut self,
            vertex: usize,
            terminal: usize,
            flow: T,
            level: &[i64],
            iter: &mut [usize],
        ) -> T {
            if vertex == terminal {
                flow
            } else {
                unsafe {
                    let mut d = T::from(0);
                    for i in *iter.get_unchecked(vertex)..self.edges.get_unchecked(vertex).len() {
                        let edge = *self.edges.get_unchecked(vertex).get_unchecked(i);
                        if edge.capacity > T::from(0)
                            && *level.get_unchecked(vertex) < *level.get_unchecked(edge.to)
                        {
                            d = self.dfs(
                                edge.to,
                                terminal,
                                std::cmp::min(flow, edge.capacity),
                                level,
                                iter,
                            );
                            if d > T::from(0) {
                                {
                                    self.edges
                                        .get_unchecked_mut(vertex)
                                        .get_unchecked_mut(i)
                                        .capacity -= d;
                                }
                                self.edges
                                    .get_unchecked_mut(edge.to)
                                    .get_unchecked_mut(edge.rev)
                                    .capacity += d;
                                break;
                            }
                        }
                        d = T::from(0);
                    }
                    d
                }
            }
        }

        pub fn max_flow(&mut self, start: usize, terminal: usize) -> T {
            let mut flow = T::from(0);
            let mut iter = vec![0usize; self.edges.len()];
            loop {
                unsafe {
                    let level = self.bfs(start);
                    if *level.get_unchecked(terminal) < 0 {
                        return flow;
                    }
                    let mut f;
                    while {
                        f = self.dfs(start, terminal, T::MAX, &level, &mut iter);
                        f > T::from(0) && f != T::MAX
                    } {
                        flow += f;
                    }
                    iter = iter.iter().map(|_| 0).collect();
                }
            }
        }
    }

    /// Return T::MAX
    pub trait Max
    where
        Self: Copy,
    {
        const MAX: Self;
    }

    impl Max for i32 {
        const MAX: Self = std::i32::MAX;
    }

    impl Max for u32 {
        const MAX: Self = std::u32::MAX;
    }

    impl Max for i64 {
        const MAX: Self = std::i64::MAX;
    }

    impl Max for u64 {
        const MAX: Self = std::u64::MAX;
    }

    impl Max for i128 {
        const MAX: Self = std::i128::MAX;
    }

    impl Max for u128 {
        const MAX: Self = std::u128::MAX;
    }

    impl Max for usize {
        const MAX: Self = std::usize::MAX;
    }

    impl Max for isize {
        const MAX: Self = std::isize::MAX;
    }

    #[cfg(test)]
    mod tests_ford_fulkerson {
        use super::*;

        #[test]
        fn for_maximum_flow_dinic1() {
            let n = 4;
            let vertics = [(0, 1, 2i64), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
            let mut flow = Dinic::new(n, &vertics);
            assert_eq!(flow.max_flow(0, 3), 3);
        }

        #[test]
        fn for_maximum_flow_dinic2() {
            let n = 2;
            let vertics = [(0, 1, 1000i64)];
            let mut flow = Dinic::new(n, &vertics);
            assert_eq!(flow.max_flow(0, 1), 1000);
        }
    }
}