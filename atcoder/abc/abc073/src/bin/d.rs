#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
        rs: [usize; r],
        loads: [(usize, usize, usize); m]
    }
    let mut shoetest = floyd_warshall_library::FloydWarshall::new(n);
    for (a, b, c) in loads {
        shoetest.add_edge(a - 1, b - 1, c);
        shoetest.add_edge(b - 1, a - 1, c);
    }
    let mut _ans = std::usize::MAX;
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

pub mod floyd_warshall_library {
    /// O(|V|^3)
    pub struct FloydWarshall<T> {
        nodes: usize,
        graph: Vec<Option<T>>,
    }

    impl<T> FloydWarshall<T>
    where
        T: std::ops::Add<Output = T> + Zero + Copy + std::cmp::Ord,
    {
        #[inline]
        pub fn new(nodes: usize) -> Self {
            let mut graph = vec![None; nodes * nodes];
            for i in 0..nodes {
                graph[i * nodes + i] = Some(T::ZERO);
            }
            Self { nodes, graph }
        }

        #[inline]
        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self.graph[from * self.nodes + to] = Some(cost);
        }

        #[inline]
        pub fn shortest_path(&self) -> Vec<Option<T>> {
            let mut graph = self.graph.clone();
            let mut now;
            for k in 0..self.nodes {
                now = graph.clone();
                for i in 0..self.nodes {
                    for j in 0..self.nodes {
                        unsafe {
                            *now.get_unchecked_mut(i * self.nodes + j) = min(
                                *now.get_unchecked(i * self.nodes + j),
                                *graph.get_unchecked(i * self.nodes + k),
                                *graph.get_unchecked(k * self.nodes + j),
                            )
                        }
                    }
                }
                graph = now;
            }
            graph
        }
    }

    #[inline]
    fn min<T: std::ops::Add<Output = T> + std::cmp::Ord>(
        dij: Option<T>,
        dik: Option<T>,
        dkj: Option<T>,
    ) -> Option<T> {
        if let Some(v2) = dik {
            if let Some(v3) = dkj {
                if let Some(v1) = dij {
                    Some(std::cmp::min(v1, v2 + v3))
                } else {
                    Some(v2 + v3)
                }
            } else {
                dij
            }
        } else {
            dij
        }
    }

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
    mod for_floyd_warshall {
        use super::*;

        #[test]
        fn for_floyd_warshall() {
            let n = 3;
            let abt = [(1, 2, 10), (2, 3, 10)];
            assert_eq!(floyd_warshall_ans(n, &abt), 10);

            let n = 5;
            let abt = [(1, 2, 12), (2, 3, 14), (3, 4, 7), (4, 5, 9), (5, 1, 18)];
            assert_eq!(floyd_warshall_ans(n, &abt), 26);

            let n = 4;
            let abt = [
                (1, 2, 1),
                (2, 3, 1),
                (3, 4, 1),
                (4, 1, 1),
                (1, 3, 1),
                (4, 2, 1),
            ];
            assert_eq!(floyd_warshall_ans(n, &abt), 1)
        }

        #[inline]
        fn floyd_warshall_ans(n: usize, abt: &[(usize, usize, usize)]) -> usize {
            let mut fw = FloydWarshall::new(n);
            for &(a, b, c) in abt {
                let x = a - 1;
                let y = b - 1;
                fw.add_edge(x, y, c);
                fw.add_edge(y, x, c);
            }
            fw.shortest_path()
                .chunks(n)
                .map(|v| v.iter().copied().max().unwrap())
                .min()
                .unwrap()
                .unwrap()
        }
    }
}
