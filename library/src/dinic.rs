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
                        *iter.get_unchecked_mut(vertex) = i;
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

        #[test]
        fn for_dinic_large() {
            let n = 100;
            let vertices = [
                (0, 1, 2),
                (0, 10, 5000),
                (0, 11, 10000),
                (1, 2, 1),
                (1, 11, 5000),
                (1, 12, 1),
                (2, 3, 5000),
                (2, 12, 1),
                (2, 13, 5000),
                (3, 4, 2),
                (3, 13, 2),
                (3, 14, 5000),
                (4, 5, 10000),
                (4, 14, 1),
                (4, 15, 2),
                (5, 6, 2),
                (5, 15, 9999),
                (5, 16, 10000),
                (6, 7, 5000),
                (6, 16, 9999),
                (6, 17, 10000),
                (7, 8, 5000),
                (7, 17, 1),
                (7, 18, 9999),
                (8, 9, 2),
                (8, 18, 2),
                (8, 19, 5000),
                (9, 19, 5000),
                (10, 11, 10000),
                (10, 20, 10000),
                (10, 21, 2),
                (11, 12, 1),
                (11, 21, 2),
                (11, 22, 9999),
                (12, 13, 1),
                (12, 22, 10000),
                (12, 23, 1),
                (13, 14, 5000),
                (13, 23, 9999),
                (13, 24, 5000),
                (14, 15, 2),
                (14, 24, 2),
                (14, 25, 9999),
                (15, 16, 9999),
                (15, 25, 2),
                (15, 26, 1),
                (16, 17, 2),
                (16, 26, 2),
                (16, 27, 10000),
                (17, 18, 1),
                (17, 27, 5000),
                (17, 28, 2),
                (18, 19, 5000),
                (18, 28, 1),
                (18, 29, 2),
                (19, 29, 10000),
                (20, 21, 2),
                (20, 30, 5000),
                (20, 31, 2),
                (21, 22, 9999),
                (21, 31, 2),
                (21, 32, 5000),
                (22, 23, 9999),
                (22, 32, 5000),
                (22, 33, 9999),
                (23, 24, 1),
                (23, 33, 10000),
                (23, 34, 9999),
                (24, 25, 5000),
                (24, 34, 10000),
                (24, 35, 1),
                (25, 26, 1),
                (25, 35, 9999),
                (25, 36, 9999),
                (26, 27, 1),
                (26, 36, 10000),
                (26, 37, 1),
                (27, 28, 2),
                (27, 37, 9999),
                (27, 38, 5000),
                (28, 29, 9999),
                (28, 38, 1),
                (28, 39, 1),
                (29, 39, 10000),
                (30, 31, 10000),
                (30, 40, 10000),
                (30, 41, 1),
                (31, 32, 1),
                (31, 41, 2),
                (31, 42, 9999),
                (32, 33, 1),
                (32, 42, 10000),
                (32, 43, 10000),
                (33, 34, 1),
                (33, 43, 9999),
                (33, 44, 10000),
                (34, 35, 1),
                (34, 44, 10000),
                (34, 45, 9999),
                (35, 36, 5000),
                (35, 45, 2),
                (35, 46, 9999),
                (36, 37, 10000),
                (36, 46, 1),
                (36, 47, 9999),
                (37, 38, 10000),
                (37, 47, 5000),
                (37, 48, 5000),
                (38, 39, 2),
                (38, 48, 10000),
                (38, 49, 1),
                (39, 49, 5000),
                (40, 41, 9999),
                (40, 50, 5000),
                (40, 51, 10000),
                (41, 42, 10000),
                (41, 51, 9999),
                (41, 52, 5000),
                (42, 43, 10000),
                (42, 52, 5000),
                (42, 53, 5000),
                (43, 44, 10000),
                (43, 53, 9999),
                (43, 54, 5000),
                (44, 45, 5000),
                (44, 54, 9999),
                (44, 55, 2),
                (45, 46, 5000),
                (45, 55, 2),
                (45, 56, 5000),
                (46, 47, 1),
                (46, 56, 10000),
                (46, 57, 5000),
                (47, 48, 10000),
                (47, 57, 5000),
                (47, 58, 9999),
                (48, 49, 10000),
                (48, 58, 9999),
                (48, 59, 5000),
                (49, 59, 9999),
                (50, 51, 2),
                (50, 60, 5000),
                (50, 61, 9999),
                (51, 52, 5000),
                (51, 61, 9999),
                (51, 62, 10000),
                (52, 53, 2),
                (52, 62, 2),
                (52, 63, 5000),
                (53, 54, 1),
                (53, 63, 10000),
                (53, 64, 9999),
                (54, 55, 5000),
                (54, 64, 10000),
                (54, 65, 10000),
                (55, 56, 9999),
                (55, 65, 2),
                (55, 66, 2),
                (56, 57, 10000),
                (56, 66, 10000),
                (56, 67, 10000),
                (57, 58, 9999),
                (57, 67, 2),
                (57, 68, 10000),
                (58, 59, 10000),
                (58, 68, 2),
                (58, 69, 10000),
                (59, 69, 5000),
                (60, 61, 9999),
                (60, 70, 5000),
                (60, 71, 5000),
                (61, 62, 5000),
                (61, 71, 5000),
                (61, 72, 5000),
                (62, 63, 5000),
                (62, 72, 10000),
                (62, 73, 10000),
                (63, 64, 10000),
                (63, 73, 2),
                (63, 74, 2),
                (64, 65, 9999),
                (64, 74, 9999),
                (64, 75, 5000),
                (65, 66, 10000),
                (65, 75, 9999),
                (65, 76, 5000),
                (66, 67, 2),
                (66, 76, 2),
                (66, 77, 1),
                (67, 68, 1),
                (67, 77, 1),
                (67, 78, 5000),
                (68, 69, 9999),
                (68, 78, 2),
                (68, 79, 9999),
                (69, 79, 9999),
                (70, 71, 1),
                (70, 80, 10000),
                (70, 81, 10000),
                (71, 72, 1),
                (71, 81, 10000),
                (71, 82, 5000),
                (72, 73, 1),
                (72, 82, 2),
                (72, 83, 9999),
                (73, 74, 2),
                (73, 83, 1),
                (73, 84, 10000),
                (74, 75, 9999),
                (74, 84, 9999),
                (74, 85, 9999),
                (75, 76, 2),
                (75, 85, 10000),
                (75, 86, 9999),
                (76, 77, 2),
                (76, 86, 10000),
                (76, 87, 5000),
                (77, 78, 10000),
                (77, 87, 9999),
                (77, 88, 2),
                (78, 79, 10000),
                (78, 88, 9999),
                (78, 89, 1),
                (79, 89, 9999),
                (80, 81, 5000),
                (80, 90, 2),
                (80, 91, 2),
                (81, 82, 2),
                (81, 91, 1),
                (81, 92, 2),
                (82, 83, 10000),
                (82, 92, 5000),
                (82, 93, 1),
                (83, 84, 5000),
                (83, 93, 10000),
                (83, 94, 1),
                (84, 85, 1),
                (84, 94, 9999),
                (84, 95, 9999),
                (85, 86, 9999),
                (85, 95, 2),
                (85, 96, 2),
                (86, 87, 10000),
                (86, 96, 1),
                (86, 97, 2),
                (87, 88, 9999),
                (87, 97, 5000),
                (87, 98, 10000),
                (88, 89, 10000),
                (88, 98, 10000),
                (88, 99, 9999),
                (89, 99, 2),
                (90, 91, 2),
                (91, 92, 10000),
                (92, 93, 2),
                (93, 94, 10000),
                (94, 95, 2),
                (95, 96, 10000),
                (96, 97, 9999),
                (97, 98, 10000),
                (98, 99, 1),
            ];
            let mut flow = Dinic::new(n, &vertices);
            assert_eq!(flow.max_flow(0, n - 1), 10002)
        }
    }
}
