// 3人の騎士と1匹の犬(https://atcoder.jp/contests/maximum-cup-2013/tasks/maximum_2013_f)
#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        knights: [(String, i64, i64); n],
        mut mages: [(String, i64, i64, i64); m]
    }
    // TODO: why??
    if n == 1 && m == 1 {
        println!(
            "{}",
            (knights[0].1 - mages[0].2).abs() + (knights[0].2 - mages[0].3).abs()
        );
        return;
    }
    mages.sort_by_key(|e| Reverse(e.1));
    let threshold = mages.iter().map(|e| e.1).nth(n - 1).unwrap_or(-1);
    let mages = mages
        .iter()
        .take_while(|e| e.1 >= threshold)
        .collect::<Vec<_>>();
    let s = 0;
    let t = knights.len() + mages.len() + 2;
    let mut flow = min_cost_flow_library::MinCostFlow::<i64>::new(t + 1);
    let upm = mages.iter().take_while(|e| e.1 > threshold).count();
    for (i, (_, x, y)) in knights.iter().enumerate() {
        flow.add_edge(s, i + 1, 1, 0);
        for (j, (_, _, mx, my)) in mages.iter().take(upm).enumerate() {
            flow.add_edge(
                i + 1,
                knights.len() + 1 + j,
                1,
                (x - mx).abs() + (y - my).abs(),
            );
        }
        for (j, (_, _, mx, my)) in mages.iter().enumerate().skip(upm) {
            flow.add_edge(
                i + 1,
                knights.len() + 1 + j,
                1,
                (x - mx).abs() + (y - my).abs(),
            );
        }
    }
    for (j, _) in mages.iter().take(upm).enumerate() {
        flow.add_edge(knights.len() + 1 + j, t, 1, 0);
    }
    for (j, _) in mages.iter().enumerate().skip(upm) {
        flow.add_edge(knights.len() + 1 + j, t - 1, 1, 0);
    }
    flow.add_edge(t - 1, t, std::cmp::max(n as i64 - upm as i64, 0), 0);
    println!(
        "{}",
        flow.min_cost_flow(s, t, std::cmp::min(knights.len(), mages.len()) as i64)
            .unwrap()
    );
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
        #[inline]
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))
        }
    }

    impl<T> PartialOrd for DirectedCost<T>
    where
        T: PartialEq + Eq + Ord,
    {
        #[inline]
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
        #[inline]
        pub fn new(vertices: usize) -> Self {
            Self {
                graph: vec![Vec::<Edge<T>>::new(); vertices],
                edges: 0,
            }
        }

        #[inline]
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

        #[inline]
        pub fn edges(&self) -> usize {
            self.edges
        }

        #[inline]
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
                        let edge = *self
                            .graph
                            .get_unchecked(*prevv.get_unchecked(v))
                            .get_unchecked(*preve.get_unchecked(v));
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
        fn for_minimum_cost_flow0() {
            let mut flow = MinCostFlow::new(8);

            flow.add_edge(0, 1, 2, 0);
            flow.add_edge(0, 2, 2, 0);
            flow.add_edge(0, 3, 2, 0);

            flow.add_edge(1, 4, 1, -10);
            flow.add_edge(1, 5, 1, -10);
            flow.add_edge(1, 6, 1, -1);
            flow.add_edge(2, 4, 1, -10);
            flow.add_edge(2, 5, 1, -10);
            flow.add_edge(2, 6, 1, -1);
            flow.add_edge(3, 4, 1, -1);
            flow.add_edge(3, 5, 1, -1);
            flow.add_edge(3, 6, 1, -10);

            flow.add_edge(4, 7, 2, 0);
            flow.add_edge(5, 7, 2, 0);
            flow.add_edge(6, 7, 2, 0);

            assert_eq!(flow.min_cost_flow(0, 7, 5), Some(-50));
        }

        #[test]
        fn for_minimum_cost_flow1() {
            let mut flow = MinCostFlow::new(2);
            flow.add_edge(0, 1, 1, 5);
            assert_eq!(flow.min_cost_flow(0, 1, 1), Some(5));

            let mut flow = MinCostFlow::new(2);
            flow.add_edge(0, 1, 2, 5);
            assert_eq!(flow.min_cost_flow(0, 1, 1), Some(5));

            let mut flow = MinCostFlow::new(2);
            flow.add_edge(0, 1, 0, 5);
            assert_eq!(flow.min_cost_flow(0, 1, 1), None);
        }

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
