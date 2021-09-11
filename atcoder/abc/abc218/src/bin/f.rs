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
    let mut ans = Vec::new();
    let mut dijkstra = dijkstra_library::Dijkstra::new(n);
    for (a, b) in &edges {
        dijkstra.add_edge(a - 1, b - 1, 1 as usize);
    }
    for i in 0..m {
        dijkstra.remove_edge(edges[i].0 - 1, edges[i].1 - 1, 1);
        let shortest = dijkstra.shortest_path(0, n - 1)[n - 1];
        if shortest == std::usize::MAX {
            ans.push("-1".to_string())
        } else {
            ans.push(shortest.to_string())
        }
        dijkstra.add_edge(edges[i].0 - 1, edges[i].1 - 1, 1);
    }
    println!("{}", ans.join("\n"))
}

fn bfs(a: &[Vec<usize>]) -> usize {
    let mut visited = vec![None; a.len()];
    visited[0] = Some(0);
    unimplemented!()
}

pub mod dijkstra_library {
    /// virifid with this(https://atcoder.jp/contests/soundhound2018-summer-qual/submissions/25632891)
    /// 1. Add edges with add_edge
    /// 2. Run self.shortest_path(from)
    /// O(|E| + |V|*log|V|)
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
        pub fn remove_edge(&mut self, from: usize, to: usize, cost: T) {
            self.edges[from] = self.edges[from]
                .iter()
                .copied()
                .filter(|e| e.to != to || e.cost != cost)
                .collect();
        }

        #[inline(always)]
        pub fn shortest_path(&mut self, start: usize, goal: usize) -> Vec<T> {
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

                if position == goal {
                    return dist;
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
