#![allow(dead_code)]

/// Dinic for maximum flow
/// O(N^2 M)
///
/// new
/// add_edge
/// max_flow
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

    impl<T> Dinic<T>
    where
        T: std::convert::From<i32>
            + std::cmp::Ord
            + Copy
            + std::ops::SubAssign
            + std::ops::AddAssign
            + Max,
    {
        #[inline]
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

        #[inline]
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

        #[inline]
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
    mod tests_dinic {
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
