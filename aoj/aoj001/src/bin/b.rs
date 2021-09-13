#![allow(unreachable_code)]

fn main() {
    let (nodes, edges, start) = input();
    let mut bellman_fold = bellman_ford_library::Bellmanford::new(nodes);
    for (u, v, c) in edges {
        bellman_fold.add_edge(u, v, c);
    }
    let dist = bellman_fold.shortest_path(start);
    println!(
        "{}",
        if let Some(d) = dist.0 {
            let mut ans = d
                .iter()
                .map(|&e| {
                    format!(
                        "{}\n",
                        if e == std::i64::MAX {
                            "INF".to_string()
                        } else {
                            e.to_string()
                        }
                    )
                })
                .collect::<String>();
            ans.pop();
            ans
        } else {
            "NEGATIVE CYCLE".to_string()
        }
    );
}

#[inline]
fn input() -> (usize, Vec<(usize, usize, i64)>, usize) {
    let (v, e, s) = {
        let e = read_line::<usize>();
        (e[0], e[1], e[2])
    };
    let mut edges = Vec::with_capacity(e);
    for _ in 0..e {
        let e = read_line::<i64>();
        edges.push((e[0] as usize, e[1] as usize, e[2]));
    }
    (v, edges, s)
}

#[inline]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

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
            assert_eq!(bellman_ford.shortest_path(start), Some(vec![0, 2, -3, -1]));

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
            assert_eq!(bellman_ford.shortest_path(start), None);
        }
    }
}
