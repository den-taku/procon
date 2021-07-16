#![allow(dead_code)]
use dinic_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut loads: [(usize, usize, i32); m],
        k: usize,
        mut points: [usize; k],
        q: usize,
        mut stones: [usize; q]
    }
    points = points.iter().map(|x| x - 1).collect();
    stones = stones.iter().map(|x| x - 1).collect();
    loads = loads.iter().map(|e| (e.0 - 1, e.1 - 1, e.2)).collect();

    // find shortest path
    let mut graph = vec![None; n * n];
    for &(u, v, cost) in &loads {
        graph[u * n + v] = Some(cost);
        graph[v * n + u] = Some(cost);
    }
    for i in 0..n {
        graph[i * n + i] = Some(0);
    }
    let mut past = graph.clone();
    let mut now = graph;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                now[i * n + j] = min(now[i * n + j], past[i * n + k], past[k * n + j]);
            }
        }
        past = now.clone();
    }
    graph = now;
    let mut upper_bound = 0;
    for i in 0..points.len() - 1 {
        upper_bound = std::cmp::max(
            upper_bound,
            graph[points[i] * n + points[i + 1]].unwrap_or(std::i32::MAX),
        );
    }
    println!(
        "{}",
        binary_search(n, upper_bound, &graph, &points, &stones).unwrap_or(0)
    );
}

#[inline]
fn binary_search(
    n: usize,
    upper_bound: i32,
    graph: &[Option<i32>],
    points: &[usize],
    stones: &[usize],
) -> Option<i32> {
    let mut upper_bound = upper_bound;
    let mut lower_bound = 0;
    while upper_bound - lower_bound > 1 {
        let est = (upper_bound + lower_bound) / 2;
        if condition(n, est, graph, points, stones) {
            upper_bound = est;
        } else {
            lower_bound = est;
        }
    }

    if condition(n, lower_bound, graph, points, stones) {
        Some(lower_bound)
    } else if condition(n, upper_bound, graph, points, stones) {
        Some(upper_bound)
    } else {
        // unreachable
        None
    }
}

#[inline]
fn condition(
    n: usize,
    est: i32,
    graph: &[Option<i32>],
    points: &[usize],
    stones: &[usize],
) -> bool {
    let mut v = Vec::with_capacity(points.len());
    for i in 0..points.len() - 1 {
        if est < graph[points[i] * n + points[i + 1]].unwrap_or(std::i32::MAX) {
            v.push(i);
        }
    }
    if v.len() == 0 {
        true
    } else if v.len() > stones.len() {
        false
    } else {
        let mut edges = vec![];
        let start = 0;
        let terminal = v.len() + stones.len() + 1;
        for i in 1..v.len() + 1 {
            edges.push((start, i, 1))
        }
        for j in v.len() + 1..v.len() + 1 + stones.len() {
            edges.push((j, terminal, 1))
        }
        for (index, &i) in v.iter().enumerate() {
            for (j, &stone) in stones.iter().enumerate() {
                if graph[stone * n + points[i + 1]].unwrap_or(std::i32::MAX) <= est {
                    edges.push((index + 1, v.len() + 1 + j, 1));
                }
            }
        }
        let mut flow = Dinic::<i64>::new(v.len() + stones.len() + 2, &edges);
        flow.max_flow(start, terminal) >= v.len() as i64
    }
}

#[inline]
fn min<T: Ord + std::ops::Add<Output = T>>(a: Option<T>, b: Option<T>, c: Option<T>) -> Option<T> {
    if let Some(num2) = b {
        if let Some(num3) = c {
            if let Some(num1) = a {
                Some(std::cmp::min(num1, num2 + num3))
            } else {
                Some(num2 + num3)
            }
        } else {
            a
        }
    } else {
        a
    }
}

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

/*
#![allow(dead_code)]
use min_cost_flow_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        loads: [(usize, usize, i32); m],
        k: usize,
        points: [usize; k],
        q: usize,
        stones: [usize; q]
    }

    // find shortest path
    let mut graph = vec![None; n * n];
    for &(u, v, cost) in &loads {
        graph[(u - 1) * n + (v - 1)] = Some(cost);
        graph[(v - 1) * n + (u - 1)] = Some(cost);
    }
    for i in 0..n {
        graph[i * n + i] = Some(0);
    }
    let mut past = graph.clone();
    let mut now = graph;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                now[i * n + j] = min(now[i * n + j], past[i * n + k], past[k * n + j]);
            }
        }
        past = now.clone();
    }

    graph = now;
    let mut flow = MinCostFlow::<i32>::new(2 * k + q);
    for i in 0..k - 1 {
        flow.add_edge(0, i + 1, 1, 0);
        flow.add_edge(k + i, 2 * k + q - 1, 1, 0);
    }
    for j in 0..q {
        flow.add_edge(2 * k - 1 + j, 2 * k + q - 1, 1, 0);
    }
    for i in 0..k - 1 {
        let s = points[i] - 1;
        let t = points[i + 1] - 1;
        let cost = graph[s * n + t].unwrap_or(std::i32::MAX);
        flow.add_edge(i + 1, k + i, 1, cost);
        for j in 0..q {
            let est = graph[(stones[j] - 1) * n + t].unwrap_or(std::i32::MAX);
            // println!("{} -> {}: {}", i, j, est);
            if est < cost {
                // println!("add: {:?}", (i, j));
                flow.add_edge(i + 1, 2 * k - 1 + j, 1, est);
            }
        }
    }
    let _cost = flow.min_cost_flow(0, 2 * k + q - 1, k as i32 - 1);
    // println!("cost {}", cost.unwrap());

    // println!("{:?}", &flow);
    let mut v = vec![0usize; k - 1];
    for (i, g) in flow.graph.iter().skip(1).take(k - 1).enumerate() {
        // println!("i: {}", i);
        for e in g {
            if e.capacity == 0 && e.to != 0 {
                // println!("{}", e.to);
                v[i] = e.to;
                break;
            }
        }
    }
    let mut max = 0;
    for (i, &e) in v.iter().enumerate() {
        // println!("{:?}", (i, e));
        if k <= e && e <= 2 * k - 2 {
            max = std::cmp::max(
                max,
                graph[(points[i] - 1) * n + (points[i + 1] - 1)].unwrap(),
            );
        } else {
            max = std::cmp::max(
                max,
                graph[(stones[e - (2 * k - 1)] - 1) * n + (points[i + 1] - 1)].unwrap(),
            );
        }
    }
    println!("{}", max);
}

fn min<T: Ord + std::ops::Add<Output = T>>(a: Option<T>, b: Option<T>, c: Option<T>) -> Option<T> {
    if let Some(num2) = b {
        if let Some(num3) = c {
            if let Some(num1) = a {
                Some(std::cmp::min(num1, num2 + num3))
            } else {
                Some(num2 + num3)
            }
        } else {
            a
        }
    } else {
        a
    }
}

pub mod min_cost_flow_library {

    #[derive(Copy, Clone, Debug)]
    pub struct Edge<T> {
        pub to: usize,
        pub capacity: T,
        pub cost: T,
        rev: usize,
    }

    #[derive(Clone, Debug)]
    pub struct MinCostFlow<T> {
        pub graph: Vec<Vec<Edge<T>>>,
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
*/
