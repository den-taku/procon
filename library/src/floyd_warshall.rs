#![allow(dead_code)]
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
        pub fn shortest_path(&self) -> (Vec<Option<T>>, Vec<usize>) {
            let mut graph = self.graph.clone();
            let mut pre: Vec<_> = (0..self.nodes).collect();
            for k in 0..self.nodes {
                for (i, pi) in pre.iter_mut().enumerate() {
                    for j in 0..self.nodes {
                        unsafe {
                            if let Some(v2) = *graph.get_unchecked(i * self.nodes + k) {
                                if let Some(v3) = *graph.get_unchecked(k * self.nodes + j) {
                                    let v = v2 + v3;
                                    if let Some(v1) = *graph.get_unchecked_mut(i * self.nodes + j) {
                                        if v1 > v {
                                            *graph.get_unchecked_mut(i * self.nodes + j) = Some(v);
                                        }
                                    } else {
                                        *graph.get_unchecked_mut(i * self.nodes + j) = Some(v);
                                    }
                                    *pi = k;
                                }
                            }
                        }
                    }
                }
            }
            (graph, pre)
        }
    }

    #[inline(always)]
    fn update<T: std::ops::Add<Output = T> + std::cmp::Ord + Copy>(
        pij: &mut Option<T>,
        pik: &mut Option<T>,
        pkj: &mut Option<T>,
    ) -> bool {
        if let Some(v2) = *pik {
            if let Some(v3) = *pkj {
                if let Some(v1) = *pij {
                    if v1 > v2 + v3 {
                        *pij = Some(v2 + v3);
                        true
                    } else {
                        false
                    }
                } else {
                    *pij = Some(v2 + v3);
                    true
                }
            } else {
                false
            }
        } else {
            false
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
                .0
                .chunks(n)
                .map(|v| v.iter().copied().max().unwrap())
                .min()
                .unwrap()
                .unwrap()
        }
    }
}
