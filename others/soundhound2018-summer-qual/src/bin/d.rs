#![allow(unreachable_code)]
#![allow(dead_code)]
use dijkstra_library::*;
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        trains: [(usize, usize, u64, u64); m]
    }
    let mut yen = Dijkstra::new(n);
    let mut dollar = Dijkstra::new(n);
    for (u, v, y, d) in trains {
        yen.add_edge(u - 1, v - 1, y);
        yen.add_edge(v - 1, u - 1, y);
        dollar.add_edge(u - 1, v - 1, d);
        dollar.add_edge(v - 1, u - 1, d);
    }
    let yen = yen.shortest_path(s - 1);
    let dollar = dollar.shortest_path(t - 1);
    let mut queue = std::collections::BinaryHeap::new();
    for i in 0..n {
        queue.push((Reverse(yen[i] + dollar[i]), i));
    }
    let mut ans = Vec::new();
    for i in 0..n {
        let mut min = std::u64::MAX;
        while let Some(&(Reverse(cand), j)) = queue.peek() {
            if j < i {
                queue.pop();
            } else {
                min = cand;
                break;
            }
        }
        ans.push((1_000_000_000_000_000 - min).to_string());
    }
    println!("{}", ans.join("\n"));
}

pub mod dijkstra_library {
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
            assert_eq!(1 + 2, 3);
        }
    }
}