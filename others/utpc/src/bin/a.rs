#![allow(dead_code)]
use min_cost_flow_library::*;

fn main() {
    let (n, m, d, cost, shop, keys, bad) = input();
    let flow0 = min_cost(n, m, d, &cost, &shop, &keys, &bad);
    println!("{}", flow0);
}

#[inline]
fn min_cost(
    n: usize,
    m: usize,
    d: usize,
    cost: &[i128],
    shop: &[usize],
    keys: &[Vec<usize>],
    bad: &[i128],
) -> i128 {
    let mut flow = MinCostFlow::<i128>::new(2 + n + m + d);
    for i in 1..n + 1 {
        flow.add_edge(0, i, 1, 0);
    }
    for j in 0..m {
        let index = n + 1 + j;
        for &i in &keys[j] {
            // flow.add_edge(i, index, std::i128::MAX, cost[j]);
            flow.add_edge(i, index, 1, cost[j]);
        }
        // flow.add_edge(index, shop[j] + n + m, std::i128::MAX, 0);
        flow.add_edge(index, shop[j] + n + m, 1, 0);
    }
    let terminal = n + m + d + 1;
    for i in n + m + 1..n + m + d + 1 {
        flow.add_edge(i, terminal, bad[i - n - m - 1], 0);
    }
    flow.min_cost_flow(0, terminal, n as i128).unwrap_or(-1)
}

#[inline]
fn input() -> (
    usize,
    usize,
    usize,
    Vec<i128>,
    Vec<usize>,
    Vec<Vec<usize>>,
    Vec<i128>,
) {
    let (n, m, d) = {
        let input = read_line::<usize>();
        (input[0], input[1], input[2])
    };
    let mut cost = Vec::with_capacity(m);
    let mut shop = Vec::with_capacity(m);
    let mut keys = Vec::with_capacity(m);
    (0..m).fold((), |_, _| {
        let input = read_line::<usize>();
        cost.push(input[0] as i128);
        shop.push(input[1]);
        keys.push(input.iter().skip(3).copied().collect::<Vec<usize>>());
    });
    let mut bad = Vec::with_capacity(d);
    (0..d).fold((), |_, _| bad.push(read_line::<i128>()[0]));
    (n, m, d, cost, shop, keys, bad)
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

pub mod min_cost_flow_library {

    #[derive(Copy, Clone, Debug)]
    struct Edge<T> {
        to: usize,
        capacity: T,
        cost: T,
        rev: usize,
    }

    #[derive(Clone, Debug)]
    pub struct MinCostFlow<T> {
        graph: Vec<Vec<Edge<T>>>,
        edges: usize,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct DirectedCost<T: Ord>(T, usize);

    impl<T> Ord for DirectedCost<T>
    where
        T: PartialEq + Eq + Ord,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))
        }
    }

    impl<T> PartialOrd for DirectedCost<T>
    where
        T: PartialEq + Eq + Ord,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> MinCostFlow<T>
    where
        T: Copy
            + std::convert::From<i32>
            + std::ops::Neg<Output = T>
            + std::cmp::Ord
            + Max
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::AddAssign
            + std::ops::SubAssign,
    {
        pub fn new(vertics: usize) -> Self {
            Self {
                graph: vec![Vec::<Edge<T>>::new(); vertics],
                edges: 0,
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, capacity: T, cost: T) {
            unsafe {
                let rev = self.graph.get_unchecked(to).len();
                self.graph.get_unchecked_mut(from).push(Edge {
                    to,
                    capacity,
                    cost,
                    rev,
                });
                let rev = self.graph.get_unchecked(from).len() - 1;
                self.graph.get_unchecked_mut(to).push(Edge {
                    to: from,
                    capacity: T::from(0),
                    cost: -cost,
                    rev,
                });
                self.edges += 1;
            }
        }

        pub fn edges(&self) -> usize {
            self.edges
        }

        pub fn min_cost_flow(&mut self, start: usize, terminal: usize, mut f: T) -> Option<T> {
            let mut res = T::from(0);
            let mut potential = vec![T::from(0); self.graph.len()];
            let mut dist = vec![T::from(0); self.graph.len()];
            let mut prevv = vec![0usize; self.graph.len()];
            let mut preve = vec![0usize; self.graph.len()];
            while f > T::from(0) {
                unsafe {
                    // dijkstra
                    let mut queue = std::collections::BinaryHeap::new();
                    dist = dist.iter().map(|_| T::MAX).collect::<Vec<_>>();
                    dist[start] = T::from(0);
                    queue.push(DirectedCost(T::from(0), start));
                    while let Some(DirectedCost(cost, position)) = queue.pop() {
                        if *dist.get_unchecked(position) < cost {
                            continue;
                        }
                        for (i, edge) in self.graph.get_unchecked(position).iter().enumerate() {
                            if edge.capacity > T::from(0)
                                && *dist.get_unchecked(edge.to)
                                    > *dist.get_unchecked(position)
                                        + edge.cost
                                        + *potential.get_unchecked(position)
                                        - *potential.get_unchecked(edge.to)
                            {
                                *dist.get_unchecked_mut(edge.to) = *dist.get_unchecked(position)
                                    + edge.cost
                                    + *potential.get_unchecked(position)
                                    - *potential.get_unchecked(edge.to);
                                *prevv.get_unchecked_mut(edge.to) = position;
                                *preve.get_unchecked_mut(edge.to) = i;
                                queue.push(DirectedCost(*dist.get_unchecked(edge.to), edge.to));
                            }
                        }
                    }
                    if *dist.get_unchecked(terminal) == T::MAX {
                        return None;
                    }
                    for (i, dis) in dist.iter().enumerate() {
                        *potential.get_unchecked_mut(i) += *dis;
                    }
                    // flush
                    let mut d = f;
                    let mut v = terminal;
                    while v != start {
                        d = std::cmp::min(
                            d,
                            self.graph
                                .get_unchecked(*prevv.get_unchecked(v))
                                .get_unchecked(*preve.get_unchecked(v))
                                .capacity,
                        );
                        v = *prevv.get_unchecked(v);
                    }
                    f -= d;
                    res += d * *potential.get_unchecked(terminal);
                    v = terminal;
                    while v != start {
                        let edge = self
                            .graph
                            .get_unchecked_mut(*prevv.get_unchecked(v))
                            .get_unchecked_mut(*preve.get_unchecked(v));
                        edge.capacity -= d;
                        let edge = self
                            .graph
                            .get_unchecked(*prevv.get_unchecked(v))
                            .get_unchecked(*preve.get_unchecked(v))
                            .clone();
                        self.graph
                            .get_unchecked_mut(v)
                            .get_unchecked_mut(edge.rev)
                            .capacity += d;
                        v = *prevv.get_unchecked(v);
                    }
                }
            }
            Some(res)
        }
    }

    /// Return T::MAX
    pub trait Max
    where
        Self: Copy,
    {
        const MAX: Self;
    }

    impl Max for i32 {
        const MAX: Self = std::i32::MAX;
    }

    impl Max for u32 {
        const MAX: Self = std::u32::MAX;
    }

    impl Max for i64 {
        const MAX: Self = std::i64::MAX;
    }

    impl Max for u64 {
        const MAX: Self = std::u64::MAX;
    }

    impl Max for i128 {
        const MAX: Self = std::i128::MAX;
    }

    impl Max for u128 {
        const MAX: Self = std::u128::MAX;
    }

    impl Max for usize {
        const MAX: Self = std::usize::MAX;
    }

    impl Max for isize {
        const MAX: Self = std::isize::MAX;
    }

    #[cfg(test)]
    mod tests_min_cost_flow {
        use super::*;

        #[test]
        fn for_minimum_cost_flow() {
            let (v, _e, f) = (4, 5, 2);
            let mut flow = MinCostFlow::new(v);
            let edges = [
                (0, 1, 2, 1),
                (0, 2, 1, 2),
                (1, 2, 1, 1),
                (1, 3, 1, 3),
                (2, 3, 2, 1),
            ];
            let _ = edges
                .iter()
                .fold((), |_, e| flow.add_edge(e.0, e.1, e.2, e.3));
            assert_eq!(flow.min_cost_flow(0, v - 1, f).unwrap_or(-1), 6);

            let (v, _e, f) = (6, 9, 3);
            let mut flow = MinCostFlow::new(v);
            let edges = [
                (0, 1, 3, 2),
                (0, 2, 2, 1),
                (1, 2, 2, 2),
                (1, 3, 3, 4),
                (2, 3, 5, 1),
                (2, 4, 6, 2),
                (3, 4, 2, 2),
                (3, 5, 6, 3),
                (4, 5, 10, 2),
            ];
            let _ = edges
                .iter()
                .fold((), |_, e| flow.add_edge(e.0, e.1, e.2, e.3));
            assert_eq!(flow.min_cost_flow(0, v - 1, f).unwrap_or(-1), 18);

            let (v, _e, f) = (6, 9, 6);
            let mut flow = MinCostFlow::new(v);
            let edges = [
                (0, 1, 3, 2),
                (0, 2, 2, 1),
                (1, 2, 2, 2),
                (1, 3, 3, 4),
                (2, 3, 5, 1),
                (2, 4, 6, 2),
                (3, 4, 2, 2),
                (3, 5, 6, 3),
                (4, 5, 10, 2),
            ];
            let _ = edges
                .iter()
                .fold((), |_, e| flow.add_edge(e.0, e.1, e.2, e.3));
            assert_eq!(flow.min_cost_flow(0, v - 1, f).unwrap_or(-1), -1);
        }
    }
}
