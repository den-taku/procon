#![allow(dead_code)]

/// FordFulkerson
///
/// new
/// add_edge
/// max_flow
pub mod ford_fulkerson_library {
    /// edges is expressed as adjacency list.
    #[derive(Clone, Debug)]
    pub struct FordFulkerson<T> {
        edges: Vec<Vec<Edge<T>>>,
        edges_size: usize,
    }

    #[derive(Copy, Clone, Debug)]
    struct Edge<T> {
        to: usize,
        capacity: T,
        rev: usize,
    }

    impl<T> FordFulkerson<T>
    where
        T: std::convert::From<i32> + Clone + Copy,
    {
        #[inline]
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

        #[inline]
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

    impl<T> FordFulkerson<T>
    where
        T: std::cmp::Ord
            + std::convert::From<i32>
            + Copy
            + std::ops::AddAssign
            + std::ops::SubAssign
            + Max,
    {
        #[inline]
        fn dfs(&mut self, vertex: usize, terminal: usize, flow: T, used: &mut [bool]) -> T {
            if vertex == terminal {
                flow
            } else {
                unsafe {
                    (*used.get_unchecked_mut(vertex)) = true;
                    let mut d = T::from(0);
                    for i in 0..self.edges.get_unchecked(vertex).len() {
                        let edge = *self.edges.get_unchecked(vertex).get_unchecked(i);
                        if !used.get_unchecked(edge.to) && edge.capacity > T::from(0) {
                            d = self.dfs(
                                edge.to,
                                terminal,
                                std::cmp::min(flow, edge.capacity),
                                used,
                            );
                            if d > T::from(0) {
                                {
                                    let edge =
                                        self.edges.get_unchecked_mut(vertex).get_unchecked_mut(i);
                                    edge.capacity -= d;
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

        #[inline]
        pub fn max_flow(&mut self, start: usize, terminal: usize) -> T {
            let mut flow = T::from(0);
            let mut used = vec![false; self.edges.len()];
            loop {
                let f = self.dfs(start, terminal, T::MAX, &mut used);
                if f == T::from(0) {
                    return flow;
                } else {
                    flow += f;
                }
                used = used.iter().map(|_| false).collect();
            }
        }
    }

    /// Return T::MAX
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

    #[cfg(test)]
    mod tests_ford_fulkerson {
        use super::*;

        #[test]
        fn for_maximum_flow() {
            let n = 4;
            let vertics = [(0, 1, 2i64), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
            let mut flow = FordFulkerson::new(n, &vertics);
            assert_eq!(flow.max_flow(0, 3), 3);
        }
    }
}
