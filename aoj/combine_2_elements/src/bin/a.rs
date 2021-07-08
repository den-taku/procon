#![allow(dead_code)]

//use proconio::{fastout, input};
use dinic_library::*;
use std::fmt::Debug;
use std::str::FromStr;

// #[fastout]
fn main() {
    // input! {
    //     n: usize,
    //     t: [[i128; 50];50]
    // }
    let (n, a, b) = {
        let input = read_line::<i32>();
        (input[0], input[1], input[2])
    };
    let n = n as usize;
    let mut ans = 0;
    let mut a_g_b = Vec::with_capacity(n);
    let mut a_l_b = Vec::with_capacity(n);
    let _ = (0..n)
        .map(|_| {
            let elem = read_line::<i32>();
            let abs = (elem[0] - elem[1]).abs();
            if abs <= a || b <= abs && abs <= 2 * a {
                ans += 1;
            } else if elem[0] > elem[1] {
                a_g_b.push((elem[0], elem[1]))
            } else {
                a_l_b.push((elem[0], elem[1]))
            }
        })
        .collect::<Vec<()>>();
    a_g_b = a_g_b;
    a_l_b = a_l_b;
    let len_g = a_g_b.len();
    let len_l = a_l_b.len();
    let mut edges = Vec::with_capacity(std::cmp::max(len_g, len_l) * 3);
    for (i, &(ax, bx)) in a_g_b.iter().enumerate() {
        edges.push((0, i + 1, 1));
        for (j, &(ay, by)) in a_l_b.iter().enumerate() {
            let abs = (ax + ay - bx - by).abs();
            if abs <= a || b <= abs && abs <= 2 * a {
                edges.push((i + 1, len_g + j + 1, 1))
            }
        }
    }
    for j in 0..a_l_b.len() {
        edges.push((len_g + j + 1, len_g + len_l + 1, 1));
    }
    println!(
        "{}",
        ans + Dinic::new(len_g + len_l + 2, &edges).max_flow(0, len_g + len_l + 1)
    );
}

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
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
