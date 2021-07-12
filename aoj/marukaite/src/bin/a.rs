#![allow(dead_code)]
use min_cost_flow_library::*;

fn main() {
    let (n, w, e, f) = input();
    let mut e_cost = vec![0i32; n];
    let mut flow = MinCostFlow::<i32>::new(2 * n + 2);
    (0..n).fold((), |_, i| {
        for (j, c) in f[i].chars().enumerate() {
            match c {
                'o' => e_cost[i] += e[i * n + j],
                '.' => (),
                _ => unreachable!(),
            }
        }
    });
    (1..n + 1).fold((), |_, i| {
        flow.add_edge(0, i, 1, 0);
        flow.add_edge(n + i, 2 * n + 1, 1, 0);
    });
    (0..n).fold((), |_, i| {
        for (j, c) in f[i].chars().enumerate() {
            match c {
                'o' => flow.add_edge(i + 1, n + j + 1, 1, e_cost[i] - e[i * n + j]),
                '.' => flow.add_edge(i + 1, n + j + 1, 1, e_cost[i] + w[i * n + j]),
                _ => unreachable!(),
            }
        }
    });
    let (cost, log) = flow.min_cost_flow(0, 2 * n + 1, n as i32);
    println!("{}", cost.unwrap());
    // println!("{}", log.len());
    let mut set = std::collections::HashSet::new();
    for s in log {
        // println!("{:?}", s);
        set.insert((s[2], s[1]));
    }
    let mut output = Vec::with_capacity(n * n);
    (0..n).fold((), |_, i| {
        for (j, c) in f[i].chars().enumerate() {
            match c {
                'o' => {
                    if let Some(_) = set.get(&(i + 1, n + j + 1)) {
                    } else {
                        output.push(format!("{} {} erase", i + 1, j + 1));
                    }
                }
                '.' => {
                    if let Some(_) = set.get(&(i + 1, n + j + 1)) {
                        output.push(format!("{} {} write", i + 1, j + 1));
                    }
                }
                _ => unreachable!(),
            }
        }
    });
    println!("{}", output.len());
    for state in &output {
        println!("{}", state);
    }
    // println!("{:?}", flow.graph());
}

#[inline]
fn input() -> (usize, Vec<i32>, Vec<i32>, Vec<String>) {
    let n = read_line::<usize>()[0];

    let mut w = Vec::with_capacity(n * n);
    let _ = (0..n).fold((), |_, _| {
        let input = read_line::<i32>();
        w.push(input);
    });
    let w = w.iter().flatten().map(|e| *e).collect::<Vec<i32>>();

    let mut e = Vec::with_capacity(n * n);
    let _ = (0..n).fold((), |_, _| {
        let input = read_line::<i32>();
        e.push(input);
    });
    let e = e.iter().flatten().map(|e| *e).collect::<Vec<i32>>();

    let mut f = Vec::with_capacity(n);
    let _ = (0..n).fold((), |_, _| {
        f.push(read_string());
    });

    (n, w, e, f)
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

#[inline]
fn read_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|s| s.chars())
        .flatten()
        .collect()
}

pub mod min_cost_flow_library {

    #[derive(Copy, Clone, Debug)]
    pub struct Edge<T> {
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
        pub fn new(vertics: usize) -> Self {
            Self {
                graph: vec![Vec::<Edge<T>>::new(); vertics],
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
        pub fn graph(&self) -> Vec<Vec<Edge<T>>> {
            self.graph.clone()
        }

        #[inline]
        pub fn min_cost_flow(
            &mut self,
            start: usize,
            terminal: usize,
            mut f: T,
        ) -> (Option<T>, Vec<Vec<usize>>) {
            let mut log = Vec::with_capacity(self.graph.len());
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
                        return (None, log);
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
                    let mut path = vec![terminal];
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
                        path.push(v);
                    }
                    log.push(path);
                }
            }
            (Some(res), log)
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
