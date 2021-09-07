pub mod bellman_ford_library {
    /// verified by this(https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5858542#1)
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
