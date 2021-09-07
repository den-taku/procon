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
        if let Some(d) = dist {
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
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                edges: Vec::new(),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self.edges.push((from, to, cost));
        }

        pub fn shortest_path(&self, start: usize) -> Option<Vec<T>> {
            let mut dist = vec![T::MAX; self.nodes];
            dist[start] = T::ZERO;
            for _ in 0..self.edges.len() + 1 {
                let mut update = false;
                for &(u, v, w) in &self.edges {
                    if dist[u] != T::MAX && dist[v] > dist[u] + w {
                        dist[v] = dist[u] + w;
                        update = true;
                    }
                }
                if !update {
                    return Some(dist);
                }
            }
            None
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
