// Cards (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1163&lang=jp)
#![allow(dead_code)]
#![allow(unreachable_code)]

fn main() {
    while let Some((blues, reds)) = input() {
        let n = blues.len();
        let m = reds.len();
        let mut edges = Vec::new();
        for i in 1..=n {
            edges.push((0, i, 1))
        }
        for (i, &b) in blues.iter().enumerate() {
            for (j, &r) in reds.iter().enumerate() {
                if gcd(b, r) != 1 {
                    edges.push((i + 1, n + j + 1, 1))
                }
            }
        }
        for j in 1..=m {
            edges.push((n + j, n + m + 1, 1))
        }
        println!(
            "{}",
            ford_fulkerson_library::FordFulkerson::new(n + m + 2, &edges).max_flow(0, n + m + 1)
        )
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut max = std::cmp::max(a, b);
    let mut min = std::cmp::min(a, b);
    while min != 0 {
        let tmp = min;
        min = max % min;
        max = tmp;
    }
    max
}

#[inline]
fn input() -> Option<(Vec<i64>, Vec<i64>)> {
    let (n, m) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    if n == 0 && m == 0 {
        None
    } else {
        let mut blues = Vec::new();
        let mut count = 0;
        while count != n {
            let mut e = read_line::<i64>();
            count += e.len();
            blues.append(&mut e);
        }
        let mut reds = Vec::new();
        let mut count = 0;
        while count != m {
            let mut e = read_line::<i64>();
            count += e.len();
            reds.append(&mut e);
        }
        Some((blues, reds))
    }
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
