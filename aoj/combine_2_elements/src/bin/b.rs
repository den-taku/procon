// Minimum Cost Flow
// (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_B&lang=jp)
use min_cost_flow_library::*;
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let (v, e, f) = {
        let input = read_line::<usize>();
        (input[0], input[1], input[2] as i32)
    };
    let mut flow = MinCostFlow::new(v);
    let _ = (0..e)
        .map(|_| {
            let elem = read_line::<usize>();
            flow.add_edge(elem[0], elem[1], elem[2] as i32, elem[3] as i32)
        })
        .collect::<Vec<()>>();
    println!("{}", flow.min_cost_flow(0, v - 1, f).unwrap_or(-1));
}

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

mod min_cost_flow_library {

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
            let rev = self.graph[to].len();
            self.graph[from].push(Edge {
                to,
                capacity,
                cost,
                rev,
            });
            let rev = self.graph[from].len() - 1;
            self.graph[to].push(Edge {
                to: from,
                capacity: T::from(0),
                cost: -cost,
                rev,
            });
            self.edges += 1;
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
                // dijkstra
                let mut queue = std::collections::BinaryHeap::new();
                dist = dist.iter().map(|_| T::MAX).collect::<Vec<_>>();
                dist[start] = T::from(0);
                queue.push(DirectedCost(T::from(0), start));
                while let Some(DirectedCost(cost, position)) = queue.pop() {
                    if dist[position] < cost {
                        continue;
                    }
                    for (i, edge) in self.graph[position].iter().enumerate() {
                        if edge.capacity > T::from(0)
                            && dist[edge.to]
                                > dist[position] + edge.cost + potential[position]
                                    - potential[edge.to]
                        {
                            dist[edge.to] = dist[position] + edge.cost + potential[position]
                                - potential[edge.to];
                            prevv[edge.to] = position;
                            preve[edge.to] = i;
                            queue.push(DirectedCost(dist[edge.to], edge.to));
                        }
                    }
                }
                if dist[terminal] == T::MAX {
                    return None;
                }
                for (i, dis) in dist.iter().enumerate() {
                    potential[i] += *dis;
                }
                // flush
                let mut d = f;
                let mut v = terminal;
                while v != start {
                    d = std::cmp::min(d, self.graph[prevv[v]][preve[v]].capacity);
                    v = prevv[v];
                }
                f -= d;
                res += d * potential[terminal];
                v = terminal;
                while v != start {
                    let edge = &mut self.graph[prevv[v]][preve[v]];
                    edge.capacity -= d;
                    let edge = self.graph[prevv[v]][preve[v]].clone();
                    self.graph[v][edge.rev].capacity += d;
                    v = prevv[v];
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
}
