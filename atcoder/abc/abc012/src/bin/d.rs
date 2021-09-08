#![allow(dead_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abt: [(usize, usize, usize); m]
    }
    let ans = dijkstra_ans(n, &abt);
    assert_eq!(ans, bellman_ford_ans(n, &abt));
    assert_eq!(ans, floyd_warshall_ans(n, &abt));
    println!("{}", ans)
}

#[inline]
fn floyd_warshall_ans(n: usize, abt: &[(usize, usize, usize)]) -> usize {
    let mut graph = vec![None; n * n];
    for &(a, b, c) in abt {
        let x = a - 1;
        let y = b - 1;
        graph[x * n + y] = Some(c);
        graph[y * n + x] = Some(c);
    }
    for i in 0..n {
        graph[i * n + i] = Some(0);
    }
    let mut now;
    for k in 0..n {
        now = graph.clone();
        for i in 0..n {
            for j in 0..n {
                now[i * n + j] = min(now[i * n + j], graph[i * n + k], graph[k * n + j])
            }
        }
        graph = now;
    }
    graph
        .chunks(n)
        .map(|v| v.iter().copied().max().unwrap())
        .min()
        .unwrap()
        .unwrap()
}

#[inline]
fn min(a: Option<usize>, b: Option<usize>, c: Option<usize>) -> Option<usize> {
    if let Some(v2) = b {
        if let Some(v3) = c {
            if let Some(v1) = a {
                Some(std::cmp::min(v1, v2 + v3))
            } else {
                Some(v2 + v3)
            }
        } else {
            a
        }
    } else {
        a
    }
}

#[inline]
fn bellman_ford_ans(n: usize, abt: &[(usize, usize, usize)]) -> usize {
    let mut bf = bellman_ford_library::Bellmanford::new(n);
    for &(x, y, c) in abt {
        bf.add_edge(x - 1, y - 1, c);
        bf.add_edge(y - 1, x - 1, c);
    }
    let mut ans = std::usize::MAX;
    for i in 0..n {
        ans = std::cmp::min(ans, *bf.shortest_path(i).0.unwrap().iter().max().unwrap())
    }
    ans
}

#[inline]
fn dijkstra_ans(n: usize, abt: &[(usize, usize, usize)]) -> usize {
    let mut dijkstra = dijkstra_library::Dijkstra::new(n);
    for &(x, y, c) in abt {
        dijkstra.add_edge(x - 1, y - 1, c);
        dijkstra.add_edge(y - 1, x - 1, c);
    }
    let mut ans = std::usize::MAX;
    for i in 0..n {
        ans = std::cmp::min(ans, *dijkstra.shortest_path(i).iter().max().unwrap())
    }
    ans
}

pub mod bellman_ford_library {
    /// verified by this(https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5858542#1)
    /// and this(https://atcoder.jp/contests/abc061/submissions/25667916)
    /// O(|E|*min(|V|, |E|))
    pub struct Bellmanford<T> {
        nodes: usize,
        edges: Vec<(usize, usize, T)>,
    }

    impl<T> Bellmanford<T>
    where
        T: Copy + Zero + Max + std::cmp::Ord + std::ops::Add<Output = T>,
    {
        #[inline(always)]
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                edges: Vec::new(),
            }
        }

        #[inline(always)]
        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self.edges.push((from, to, cost));
        }

        #[inline(always)]
        pub fn shortest_path(&self, start: usize) -> (Option<Vec<T>>, Vec<usize>) {
            let mut dist = vec![T::MAX; self.nodes];
            let mut pre = vec![0; self.nodes];
            dist[start] = T::ZERO;
            for _ in 0..std::cmp::min(self.nodes, self.edges.len() + 1) {
                let mut update = false;
                for &(u, v, w) in &self.edges {
                    if dist[u] != T::MAX && dist[v] > dist[u] + w {
                        dist[v] = dist[u] + w;
                        pre[v] = u;
                        update = true;
                    }
                }
                if !update {
                    return (Some(dist), pre);
                }
            }
            (None, pre)
        }

        #[inline(always)]
        pub fn shortest_path_complete(&self, start: usize) -> (Vec<Option<T>>, Vec<usize>) {
            let mut dist = vec![Some(T::MAX); self.nodes];
            let mut pre = vec![0; self.nodes];
            dist[start] = Some(T::ZERO);
            for _ in 0..std::cmp::min(self.nodes - 1, self.edges.len()) {
                let mut update = false;
                for &(u, v, w) in &self.edges {
                    if dist[u].unwrap() != T::MAX && dist[v].unwrap() > dist[u].unwrap() + w {
                        dist[v] = Some(dist[u].unwrap() + w);
                        pre[v] = u;
                        update = true;
                    }
                }
                if !update {
                    return (dist, pre);
                }
            }
            let mut negative = vec![false; self.nodes];
            let mut update = false;
            for &(u, v, w) in &self.edges {
                if dist[u].unwrap() != T::MAX && dist[v].unwrap() > dist[u].unwrap() + w {
                    dist[v] = Some(dist[u].unwrap() + w);
                    pre[v] = u;
                    negative[v] = true;
                    update = true;
                }
                if negative[u] {
                    negative[v] = true;
                }
            }
            if !update {
                return (dist, pre);
            }
            for _ in 0..std::cmp::min(self.nodes - 1, self.edges.len() - 1) {
                for &(u, v, w) in &self.edges {
                    if dist[u].unwrap() != T::MAX && dist[v].unwrap() > dist[u].unwrap() + w {
                        dist[v] = Some(dist[u].unwrap() + w);
                        pre[v] = u;
                        negative[v] = true;
                    }
                    if negative[u] {
                        negative[v] = true;
                    }
                }
            }
            (
                negative
                    .into_iter()
                    .zip(dist.into_iter())
                    .map(|(b, d)| if b { None } else { d })
                    .collect(),
                pre,
            )
        }
    }

    pub trait Max {
        const MAX: Self;
    }

    macro_rules! impl_max {
        ( $($e:ident),* ) => {
            $(
                impl Max for $e {
                    const MAX: Self = std::$e::MAX;
                }
            )*
        };
    }

    impl_max!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    pub trait Zero {
        const ZERO: Self;
    }

    macro_rules! impl_zero {
        ( $($e:ty),* ) => {
            $(
                impl Zero for $e {
                    const ZERO: Self = 0;
                }
            )*
        };
    }

    impl_zero!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    #[cfg(test)]
    mod tests_bellman_ford {
        use super::*;

        #[test]
        fn for_bellman_ford() {
            let nodes = 4;
            let start = 0;
            let edges = [(0, 1, 2i64), (0, 2, 3), (1, 2, -5), (1, 3, 1), (2, 3, 2)];
            let mut bellman_ford = Bellmanford::new(nodes);
            for &(u, v, c) in edges.iter() {
                bellman_ford.add_edge(u, v, c);
            }
            assert_eq!(
                bellman_ford.shortest_path(start).0,
                Some(vec![0, 2, -3, -1])
            );

            let nodes = 4;
            let start = 0;
            let edges = [
                (0, 1, 2),
                (0, 2, 3),
                (1, 2, -5),
                (1, 3, 1),
                (2, 3, 2),
                (3, 1, 0),
            ];
            let mut bellman_ford = Bellmanford::new(nodes);
            for &(u, v, c) in edges.iter() {
                bellman_ford.add_edge(u, v, c);
            }
            assert_eq!(bellman_ford.shortest_path(start).0, None);
        }
    }
}

pub mod dijkstra_library {
    /// virifid with this(https://atcoder.jp/contests/soundhound2018-summer-qual/submissions/25632891)
    /// 1. Add edges with add_edge
    /// 2. Run self.shortest_path(from)
    #[derive(Clone, Debug)]
    pub struct Dijkstra<T> {
        nodes: usize,
        edges: Vec<Vec<Edge<T>>>,
    }

    impl<T> Dijkstra<T>
    where
        T: Clone + Max + Zero + std::cmp::Ord + std::ops::Add<Output = T> + Copy,
    {
        #[inline(always)]
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                edges: vec![Vec::new(); nodes],
            }
        }

        #[inline(always)]
        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            if to >= self.nodes || from >= self.nodes {
                panic!("add_edge: out of bound.")
            }
            unsafe { self.edges.get_unchecked_mut(from).push(Edge { to, cost }) };
        }

        #[inline(always)]
        pub fn shortest_path(&mut self, start: usize) -> Vec<T> {
            if start >= self.nodes {
                panic!("shortest_path: start is out of bound.")
            }
            let mut dist: Vec<_> = (0..self.nodes).map(|_| T::MAX).collect();

            let mut heap = std::collections::BinaryHeap::new();

            unsafe {
                *dist.get_unchecked_mut(start) = T::ZERO;
            }
            heap.push(State {
                cost: T::ZERO,
                position: start,
            });

            while let Some(State { cost, position }) = heap.pop() {
                if cost > unsafe { *dist.get_unchecked(position) } {
                    continue;
                }

                unsafe {
                    for edge in self.edges.get_unchecked(position) {
                        let next = State {
                            cost: cost + edge.cost,
                            position: edge.to,
                        };

                        if next.cost < *dist.get_unchecked(next.position) {
                            heap.push(next);
                            // Relaxation, we have now found a better way
                            *dist.get_unchecked_mut(next.position) = next.cost;
                        }
                    }
                }
            }

            dist
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct Edge<T> {
        to: usize,
        cost: T,
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State<T> {
        cost: T,
        position: usize,
    }

    impl<T> Ord for State<T>
    where
        T: std::cmp::Ord,
    {
        #[inline(always)]
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    impl<T> PartialOrd for State<T>
    where
        T: std::cmp::Ord + std::cmp::PartialOrd,
    {
        #[inline(always)]
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    pub trait Max {
        const MAX: Self;
    }

    macro_rules! impl_max {
        ( $($e:ident),* ) => {
            $(
                impl Max for $e {
                    const MAX: Self = std::$e::MAX;
                }
            )*
        };
    }

    impl_max!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    pub trait Zero {
        const ZERO: Self;
    }

    macro_rules! impl_zero {
        ( $($e:ty),* ) => {
            $(
                impl Zero for $e {
                    const ZERO: Self = 0;
                }
            )*
        };
    }

    impl_zero!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    #[cfg(test)]
    mod tests_dijkstra {
        use super::*;

        #[test]
        fn for_dijkstra() {
            // This is the directed graph we're going to use.
            // The node numbers correspond to the different states,
            // and the edge weights symbolize the cost of moving
            // from one node to another.
            // Note that the edges are one-way.
            //
            //                  7
            //          +-----------------+
            //          |                 |
            //          v   1        2    |  2
            //          0 -----> 1 -----> 3 ---> 4
            //          |        ^        ^      ^
            //          |        | 1      |      |
            //          |        |        | 3    | 1
            //          +------> 2 -------+      |
            //           10      |               |
            //                   +---------------+
            //
            // The graph is represented as an adjacency list where each index,
            // corresponding to a node value, has a list of outgoing edges.
            // Chosen for its efficiency.
            let mut dijkstra = Dijkstra::new(5);

            // Node 0
            dijkstra.add_edge(0, 2, 10);
            dijkstra.add_edge(0, 1, 1);
            // Node 1
            dijkstra.add_edge(1, 3, 2);
            // Node 2
            dijkstra.add_edge(2, 1, 1);
            dijkstra.add_edge(2, 3, 3);
            dijkstra.add_edge(2, 4, 1);
            // Node 3
            dijkstra.add_edge(3, 0, 7);
            dijkstra.add_edge(3, 4, 2);
            // Node 4

            assert_eq!(dijkstra.shortest_path(0)[1], 1);
            assert_eq!(dijkstra.shortest_path(0)[3], 3);
            assert_eq!(dijkstra.shortest_path(3)[0], 7);
            assert_eq!(dijkstra.shortest_path(0)[4], 5);
            assert_eq!(dijkstra.shortest_path(4)[0], std::usize::MAX);
        }
    }
}
