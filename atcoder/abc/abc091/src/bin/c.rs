#![allow(dead_code)]
use hungarian_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        reds: [(usize, usize); n],
        blues: [(usize, usize); n]
    }
    let mut vertecs = Vec::with_capacity((n + 1) * (n + 2));
    (0..n).fold((), |_, i| {
        vertecs.push((0, i + 1, 1));
        vertecs.push((i + n + 1, 2 * n + 1, 1));
    });
    for (i, red) in reds.iter().enumerate() {
        for (j, blue) in blues.iter().enumerate() {
            if red.0 < blue.0 && red.1 < blue.1 {
                vertecs.push((i + 1, j + 1 + n, 1));
            }
        }
    }
    println!("{}", {
        Dinic::new(2 * n + 2, &vertecs).max_flow(0, 2 * n + 1)
    });
}

pub mod hungarian_library {

    #[derive(Clone, Debug)]
    pub struct Hungarian {
        pub graph: Graph,
    }

    impl Hungarian {
        pub fn new(graph: Graph) -> Self {
            Self { graph }
        }
    }

    impl Hungarian {
        #[inline]
        pub fn maximum_matching_with_graph<T>(&self) -> (T, Graph)
        where
            T: std::convert::From<i32> + std::ops::AddAssign,
        {
            let mut matchings = T::from(0);
            let mut matching = Graph::new(self.graph.vertics());
            let mut match_flag = vec![false; self.graph.vertics()];
            for i in 0..self.graph.vertics() {
                if let Some(path) = self.find_augmenting_path(i, &match_flag, &matching) {
                    let mut past;
                    let mut past_match = false;
                    unsafe { past = *path.get_unchecked(0) }
                    unsafe {
                        *match_flag.get_unchecked_mut(past) = true;
                        let len = path.len();
                        *match_flag.get_unchecked_mut(*path.get_unchecked(len - 1)) = true;
                    }
                    for &e in path.iter().skip(1) {
                        if past_match {
                            matching.remove_edge(past, e)
                        } else {
                            matching.add_edge(past, e)
                        }
                        past = e;
                        past_match ^= true;
                    }
                    matchings += T::from(1);
                }
            }
            (matchings, matching)
        }

        #[inline]
        fn find_augmenting_path(
            &self,
            start: usize,
            match_flag: &[bool],
            matching: &Graph,
        ) -> Option<Vec<usize>> {
            unsafe {
                if *match_flag.get_unchecked(start) {
                    None
                } else {
                    let v = self.graph.vertics();
                    let mut visited = vec![0; v];
                    *visited.get_unchecked_mut(start) = 1;
                    let mut turn = 2;
                    let mut queue_s = std::collections::VecDeque::with_capacity(v);
                    let mut queue_t = std::collections::VecDeque::with_capacity(v);
                    queue_s.push_back(start);
                    let mut end;
                    'out: loop {
                        if visited.iter().fold(1, |m, e| m * e) != 0 {
                            return None;
                        }
                        if queue_s.is_empty() {
                            for e in queue_t {
                                'inner: for i in 0..v {
                                    if *visited.get_unchecked(i) == 0
                                        && *match_flag.get_unchecked(e)
                                        && *self.graph.incidence_matrix.get_unchecked(e * v + i)
                                    {
                                        *visited.get_unchecked_mut(i) = turn;
                                        queue_s.push_back(i);
                                        break 'inner;
                                    }
                                }
                            }
                            queue_t = std::collections::VecDeque::new();
                        } else {
                            for e in queue_s {
                                for i in 0..v {
                                    if *visited.get_unchecked(i) == 0
                                        && *self.graph.incidence_matrix.get_unchecked(e * v + i)
                                    {
                                        queue_t.push_back(i);
                                        *visited.get_unchecked_mut(i) = turn;
                                        if !*match_flag.get_unchecked(i) {
                                            end = i;
                                            break 'out;
                                        }
                                    }
                                }
                            }
                            queue_s = std::collections::VecDeque::new();
                        }
                        turn += 1;
                    }
                    let mut path = Vec::with_capacity(v);
                    path.push(end);
                    let mut m_flag = false;
                    while turn > 1 {
                        *visited.get_unchecked_mut(end) = 0;
                        turn -= 1;
                        if m_flag {
                            for i in 0..v {
                                if *visited.get_unchecked(i) == turn
                                    && *matching.incidence_matrix.get_unchecked(end * v + i)
                                {
                                    path.push(i);
                                    end = i;
                                    break;
                                }
                            }
                        } else {
                            for i in 0..v {
                                if *visited.get_unchecked(i) == turn {
                                    path.push(i);
                                    end = i;
                                    break;
                                }
                            }
                        }
                        m_flag ^= true;
                    }
                    Some(path)
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Graph {
        incidence_matrix: Vec<bool>,
        vertics: usize,
        edges: usize,
    }

    impl Graph {
        #[inline]
        pub fn new(vertics: usize) -> Self {
            Self {
                incidence_matrix: vec![false; vertics * vertics],
                vertics,
                edges: 0,
            }
        }

        #[inline]
        pub fn add_edge(&mut self, a: usize, b: usize) {
            let v = self.vertics();
            if a >= v || b >= v {
                panic!("error: vertix is over |V|")
            }
            unsafe {
                if *self.incidence_matrix.get_unchecked(a * v + b) {
                    panic!("error: edge is already existed.")
                }
                *self.incidence_matrix.get_unchecked_mut(a * v + b) = true;
                *self.incidence_matrix.get_unchecked_mut(b * v + a) = true;
            }
            self.edges += 1;
        }

        #[inline]
        pub fn remove_edge(&mut self, a: usize, b: usize) {
            let v = self.vertics();
            if a >= v || b >= v {
                panic!("error: vertix is over |V|")
            }
            unsafe {
                if !*self.incidence_matrix.get_unchecked(a * v + b) {
                    panic!("error: edge is already removed.")
                }
                *self.incidence_matrix.get_unchecked_mut(a * v + b) = false;
                *self.incidence_matrix.get_unchecked_mut(b * v + a) = false;
            }
            self.edges -= 1;
        }

        #[inline]
        pub fn vertics(&self) -> usize {
            self.vertics
        }

        #[inline]
        pub fn edges(&self) -> usize {
            self.edges
        }
    }

    #[cfg(test)]
    mod tests_hungarian {
        use super::*;

        #[test]
        fn for_graph() {
            let mut graph = Graph::new(3);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 2);
            assert_eq!(graph.edges(), 3);
        }

        #[test]
        fn for_hungarian() {
            let mut graph = Graph::new(3);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 2);
            let hungarian = Hungarian::new(graph);
            assert_eq!(hungarian.maximum_matching_with_graph::<i32>().0, 1);
            let mut graph = Graph::new(4);
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            let hungarian = Hungarian::new(graph);
            assert_eq!(hungarian.maximum_matching_with_graph::<i32>().0, 2);
        }
    }
}


// #![allow(dead_code)]
// use dinic_library::*;
// use proconio::{fastout, input};

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         reds: [(usize, usize); n],
//         blues: [(usize, usize); n]
//     }
//     let mut vertecs = Vec::with_capacity((n + 1) * (n + 2));
//     (0..n).fold((), |_, i| {
//         vertecs.push((0, i + 1, 1));
//         vertecs.push((i + n + 1, 2 * n + 1, 1));
//     });
//     for (i, red) in reds.iter().enumerate() {
//         for (j, blue) in blues.iter().enumerate() {
//             if red.0 < blue.0 && red.1 < blue.1 {
//                 vertecs.push((i + 1, j + 1 + n, 1));
//             }
//         }
//     }
//     println!("{}", {
//         Dinic::new(2 * n + 2, &vertecs).max_flow(0, 2 * n + 1)
//     });
// }

// pub mod dinic_library {
//     /// edges is expressed as adjacency list.
//     #[derive(Clone, Debug)]
//     pub struct Dinic<T> {
//         edges: Vec<Vec<Edge<T>>>,
//         edges_size: usize,
//     }

//     #[derive(Copy, Clone, Debug)]
//     struct Edge<T> {
//         to: usize,
//         capacity: T,
//         rev: usize,
//     }

//     impl<T> Dinic<T>
//     where
//         T: std::convert::From<i32> + Clone + Copy,
//     {
//         fn add_edge(&mut self, from: usize, to: usize, capacity: T) {
//             unsafe {
//                 let rev = self.edges.get_unchecked(to).len();
//                 self.edges
//                     .get_unchecked_mut(from)
//                     .push(Edge { to, capacity, rev });
//                 let rev = self.edges.get_unchecked(from).len() - 1;
//                 self.edges.get_unchecked_mut(to).push(Edge {
//                     to: from,
//                     capacity: T::from(0i32),
//                     rev,
//                 });
//             }
//         }

//         pub fn new(n: usize, edges: &[(usize, usize, T)]) -> Self {
//             let mut container = Self {
//                 edges: vec![vec![]; n],
//                 edges_size: edges.len(),
//             };
//             for &(from, to, capacity) in edges {
//                 container.add_edge(from, to, capacity);
//             }
//             container
//         }
//     }

//     impl<T> Dinic<T>
//     where
//         T: std::convert::From<i32>
//             + std::cmp::Ord
//             + Copy
//             + std::ops::SubAssign
//             + std::ops::AddAssign
//             + Max,
//     {
//         fn bfs(&self, start: usize) -> Vec<i64> {
//             unsafe {
//                 let mut level = vec![-1; self.edges.len()];
//                 let mut queue = std::collections::VecDeque::with_capacity(self.edges_size);
//                 *level.get_unchecked_mut(start) = 0;
//                 queue.push_back(start);
//                 while !queue.is_empty() {
//                     let vertex = queue.pop_front().unwrap();
//                     (0..self.edges.get_unchecked(vertex).len()).fold((), |_, i| {
//                         let edge = self.edges.get_unchecked(vertex).get_unchecked(i);
//                         if edge.capacity > T::from(0) && *level.get_unchecked(edge.to) < 0 {
//                             *level.get_unchecked_mut(edge.to) = *level.get_unchecked(vertex) + 1;
//                             queue.push_back(edge.to);
//                         }
//                     });
//                 }
//                 level
//             }
//         }

//         fn dfs(
//             &mut self,
//             vertex: usize,
//             terminal: usize,
//             flow: T,
//             level: &[i64],
//             iter: &mut [usize],
//         ) -> T {
//             if vertex == terminal {
//                 flow
//             } else {
//                 unsafe {
//                     let mut d = T::from(0);
//                     for i in *iter.get_unchecked(vertex)..self.edges.get_unchecked(vertex).len() {
//                         let edge = *self.edges.get_unchecked(vertex).get_unchecked(i);
//                         if edge.capacity > T::from(0)
//                             && *level.get_unchecked(vertex) < *level.get_unchecked(edge.to)
//                         {
//                             d = self.dfs(
//                                 edge.to,
//                                 terminal,
//                                 std::cmp::min(flow, edge.capacity),
//                                 level,
//                                 iter,
//                             );
//                             if d > T::from(0) {
//                                 {
//                                     self.edges
//                                         .get_unchecked_mut(vertex)
//                                         .get_unchecked_mut(i)
//                                         .capacity -= d;
//                                 }
//                                 self.edges
//                                     .get_unchecked_mut(edge.to)
//                                     .get_unchecked_mut(edge.rev)
//                                     .capacity += d;
//                                 break;
//                             }
//                         }
//                         d = T::from(0);
//                     }
//                     d
//                 }
//             }
//         }

//         pub fn max_flow(&mut self, start: usize, terminal: usize) -> T {
//             let mut flow = T::from(0);
//             let mut iter = vec![0usize; self.edges.len()];
//             loop {
//                 unsafe {
//                     let level = self.bfs(start);
//                     if *level.get_unchecked(terminal) < 0 {
//                         return flow;
//                     }
//                     let mut f;
//                     while {
//                         f = self.dfs(start, terminal, T::MAX, &level, &mut iter);
//                         f > T::from(0) && f != T::MAX
//                     } {
//                         flow += f;
//                     }
//                     iter = iter.iter().map(|_| 0).collect();
//                 }
//             }
//         }
//     }

//     /// Return T::MAX
//     pub trait Max
//     where
//         Self: Copy,
//     {
//         const MAX: Self;
//     }

//     impl Max for i32 {
//         const MAX: Self = std::i32::MAX;
//     }

//     impl Max for u32 {
//         const MAX: Self = std::u32::MAX;
//     }

//     impl Max for i64 {
//         const MAX: Self = std::i64::MAX;
//     }

//     impl Max for u64 {
//         const MAX: Self = std::u64::MAX;
//     }

//     impl Max for i128 {
//         const MAX: Self = std::i128::MAX;
//     }

//     impl Max for u128 {
//         const MAX: Self = std::u128::MAX;
//     }

//     impl Max for usize {
//         const MAX: Self = std::usize::MAX;
//     }

//     impl Max for isize {
//         const MAX: Self = std::isize::MAX;
//     }

//     #[cfg(test)]
//     mod tests_ford_fulkerson {
//         use super::*;

//         #[test]
//         fn for_maximum_flow_dinic1() {
//             let n = 4;
//             let vertics = [(0, 1, 2i64), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
//             let mut flow = Dinic::new(n, &vertics);
//             assert_eq!(flow.max_flow(0, 3), 3);
//         }

//         #[test]
//         fn for_maximum_flow_dinic2() {
//             let n = 2;
//             let vertics = [(0, 1, 1000i64)];
//             let mut flow = Dinic::new(n, &vertics);
//             assert_eq!(flow.max_flow(0, 1), 1000);
//         }
//     }
// }