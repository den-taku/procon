// Strongly Connected Components (https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/3/GRL_3_C) 00.00
#![allow(dead_code)]
#![allow(unreachable_code)]
use tree_dist_library::*;

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock(), 4096);
    input! {
        sc = sc,
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
        q: usize,
        queries: [(usize, usize); q]
    }
    let mut com = TreeDistStrongly::new(n);
    for (u, v) in edges {
        com.add_edge(u, v)
    }
    let id = com.strongly_connected_component();
    let mut ans = Vec::with_capacity(q);
    for (u, v) in queries {
        if id[u] == id[v] {
            ans.push("1")
        } else {
            ans.push("0")
        }
    }
    println!("{}", ans.join("\n"))
}

#[macro_export]
macro_rules! input{
    (sc=$sc:expr,$($r:tt)*)=>{
        input_inner!{$sc,$($r)*}
    };
    ($($r:tt)*)=>{
        let mut sc=fast_input::Scanner::new(std::io::stdin().lock(),4096);
        input_inner!{sc,$($r)*}
    };
}

#[macro_export]
macro_rules! input_inner{
    ($sc:expr)=>{};
    ($sc:expr,)=>{};
    ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{
        let $var=read_value!($sc,$t);
        input_inner!{$sc $($r)*}
    };
}

#[macro_export]
macro_rules! read_value{
    ($sc:expr,($($t:tt),*))=>{
        ($(read_value!($sc,$t)),*)
    };
    ($sc:expr,[$t:tt;$len:expr])=>{
        (0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()
    };
    ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
    ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
    ($sc:expr,$t:ty)=>{$sc.next::<$t>()};
}

pub struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}
impl Scanner {
    pub fn new<R: std::io::Read>(mut reader: R, estimated: usize) -> Self {
        let mut buf = Vec::with_capacity(estimated);
        let _ = std::io::copy(&mut reader, &mut buf).unwrap();
        if buf.last() != Some(&b'\n') {
            panic!("{}", 0);
        }
        Scanner { buf, pos: 0 }
    }
    #[inline]
    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        let mut start = None;
        loop {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) | (b'\n', false) | (b'\r', false) => self.pos += 1,
                (_, false) => start = Some(self.pos),
            }
        }
        let target = &self.buf[start.unwrap()..self.pos];
        unsafe { std::str::from_utf8_unchecked(target) }
            .parse()
            .unwrap()
    }
}

/// Manage Tree
///
/// new
/// add_edge
/// remove_edge
/// distance
/// count_distance_for_circuit
/// count_subtree_from
/// eulerian_walk
/// strongly_connected_components
pub mod tree_dist_library {
    /// verified by this(https://atcoder.jp/contests/abc218/submissions/25794641)
    pub struct TreeDist {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
    }

    pub struct TreeDistStrongly {
        nodes: usize,
        pub adjacent: Vec<Vec<usize>>,
        pub rev: Vec<Vec<usize>>,
    }

    impl TreeDist {
        pub fn count_path<T>(&self, start: usize, zero: T, one: T) -> (Vec<T>, Vec<Option<usize>>)
        where
            T: Copy + std::ops::AddAssign,
        {
            let mut distance = vec![None; self.nodes];
            distance[start] = Some(0);
            let mut count = vec![zero; self.nodes];
            count[start] += one;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(u) = queue.pop_front() {
                for &v in &self.adjacent[u] {
                    if distance[v].is_none() {
                        distance[v] = Some(distance[u].unwrap() + 1);
                        count[v] = count[u];
                        queue.push_back(v);
                    } else if let Some(d) = distance[v] {
                        let dis = distance[u].unwrap();
                        if d == dis + 1 {
                            let tmp = count[u];
                            count[v] += tmp;
                        }
                    }
                }
            }
            (count, distance)
        }
    }

    impl TreeDist {
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
        }

        pub fn eulerian_walk(&self) -> std::collections::LinkedList<(usize, usize)> {
            let mut edges = vec![std::collections::HashSet::new(); self.nodes];
            for (i, v) in self.adjacent.iter().enumerate() {
                for &e in v {
                    edges[i].insert(e);
                }
            }
            self.eulerian_walk_rec(0, &mut edges)
        }

        fn eulerian_walk_rec(
            &self,
            start: usize,
            edges: &mut [std::collections::HashSet<usize>],
        ) -> std::collections::LinkedList<(usize, usize)> {
            let mut path = vec![start];
            let mut v = start;
            while !edges[v].is_empty() {
                let &w = edges[v].iter().last().unwrap();
                edges[v].remove(&w);
                path.push(w);
                v = w;
            }
            let mut walk = std::collections::LinkedList::new();
            if path.len() >= 2 {
                walk.push_back((path[0], path[1]))
            }
            for (&v, &w) in path.iter().zip(path.iter().skip(1)).skip(1) {
                walk.append(&mut self.eulerian_walk_rec(v, edges));
                walk.push_back((v, w));
            }
            walk
        }
    }

    impl TreeDistStrongly {
        /// return strongly connected component id.
        /// if graph has moderate topological order, then return unique id
        pub fn strongly_connected_component(&self) -> Vec<usize> {
            let mut visited = vec![false; self.nodes];
            let mut order = vec![0; self.nodes];
            let mut inverse = vec![0; self.nodes];
            let mut n = 0;
            for v in 0..self.nodes {
                if !visited[v] {
                    self.visit1(v, &mut n, &mut order, &mut inverse, &mut visited);
                }
            }
            let mut visited = vec![false; self.nodes];
            let mut comp = vec![0; self.nodes];
            let mut k = 0;
            for i in (0..self.nodes).rev() {
                if !visited[inverse[i]] {
                    self.visit2(inverse[i], k, &mut comp, &mut visited);
                    k += 1;
                }
            }
            comp
        }

        pub fn eulerian_walk(&self) -> std::collections::LinkedList<(usize, usize)> {
            let mut edges = vec![std::collections::HashSet::new(); self.nodes];
            for (i, v) in self.adjacent.iter().enumerate() {
                for &e in v {
                    edges[i].insert(e);
                }
            }
            self.eulerian_walk_rec(0, &mut edges)
        }

        fn eulerian_walk_rec(
            &self,
            start: usize,
            edges: &mut [std::collections::HashSet<usize>],
        ) -> std::collections::LinkedList<(usize, usize)> {
            let mut path = vec![start];
            let mut v = start;
            while !edges[v].is_empty() {
                let &w = edges[v].iter().last().unwrap();
                edges[v].remove(&w);
                path.push(w);
                v = w;
            }
            let mut walk = std::collections::LinkedList::new();
            if path.len() >= 2 {
                walk.push_back((path[0], path[1]))
            }
            for (&v, &w) in path.iter().zip(path.iter().skip(1)).skip(1) {
                walk.append(&mut self.eulerian_walk_rec(v, edges));
                walk.push_back((v, w));
            }
            walk
        }

        fn visit1(
            &self,
            v: usize,
            n: &mut usize,
            order: &mut [usize],
            inverse: &mut [usize],
            visited: &mut [bool],
        ) {
            visited[v] = true;
            for &w in &self.adjacent[v] {
                if !visited[w] {
                    self.visit1(w, n, order, inverse, visited);
                }
            }
            order[v] = *n;
            inverse[*n] = v;
            *n += 1;
        }

        fn visit2(&self, w: usize, k: usize, comp: &mut [usize], visited: &mut [bool]) {
            visited[w] = true;
            for &v in &self.rev[w] {
                if !visited[v] {
                    self.visit2(v, k, comp, visited);
                }
            }
            comp[w] = k;
        }

        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                adjacent: vec![Vec::new(); nodes],
                rev: vec![Vec::new(); nodes],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u].push(v);
            self.rev[v].push(u);
        }

        pub fn remove_edge(&mut self, u: usize, v: usize) {
            self.adjacent[u] = self.adjacent[u]
                .iter()
                .copied()
                .filter(|&e| e != v)
                .collect();
            self.rev[v] = self.rev[v].iter().copied().filter(|&e| e != u).collect();
        }

        pub fn distance(&self, start: usize) -> (Vec<Option<usize>>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            dist[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        queue.push_back(e);
                        pre[e] = s;
                    }
                }
            }
            (dist, pre)
        }

        pub fn count_distance_for_circuit(
            &self,
            start: usize,
            terminal: usize,
        ) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
            let mut dist = vec![None; self.nodes];
            let mut pre = (0..self.nodes).collect::<Vec<_>>();
            let mut count = vec![0; self.nodes];
            dist[start] = Some(0);
            count[start] = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            while let Some(s) = queue.pop_front() {
                for &e in &self.adjacent[s] {
                    if dist[e].is_none() {
                        dist[e] = Some(dist[s].unwrap() + 1);
                        pre[e] = s;
                        count[e] = count[s];
                        if e != terminal {
                            queue.push_back(e);
                        }
                    } else if let Some(d) = dist[e] {
                        if d == dist[s].unwrap() + 1 {
                            count[e] += count[s];
                            if e != terminal {
                                queue.push_back(e);
                            }
                        }
                    }
                }
            }
            (dist, pre, count)
        }

        /// virifid (https://atcoder.jp/contests/abc220/submissions/26208107)
        /// count subtree's size
        pub fn count_subtree_from(&self, start: usize) -> Vec<usize> {
            let mut count = vec![0; self.nodes];
            self.rec_subtree(self.nodes, start, &mut count);
            count
        }

        fn rec_subtree(&self, from: usize, now: usize, count: &mut [usize]) -> usize {
            let mut sum = 1;
            for &v in &self.adjacent[now] {
                if v == from {
                    continue;
                }
                sum += self.rec_subtree(now, v, count);
            }
            count[now] = sum;
            sum
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

    pub trait One {
        const ONE: Self;
    }

    macro_rules! impl_one {
            ( $($e:ty),* ) => {
                $(
                    impl One for $e {
                        const ONE: Self = 1;
                    }
                )*
            };
        }

    impl_one!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    #[cfg(test)]
    mod tests_tree_dist {
        use super::*;

        #[test]
        fn for_count() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDist::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDist::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDist::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }

        #[test]
        fn for_count_strongly() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0],
                    vec![1, 1, 2, 1, 1]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.count_distance_for_circuit(0, n - 1),
                (
                    vec![Some(0), Some(1), None, None],
                    vec![0, 0, 2, 3],
                    vec![1, 1, 0, 0]
                )
            )
        }

        #[test]
        fn for_dist_strongly() {
            let n = 5;
            let edges = [
                (1, 2),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 2),
                (4, 3),
            ];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in edges.iter() {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (
                    vec![Some(0), Some(1), Some(2), Some(1), Some(1)],
                    vec![0, 0, 1, 0, 0]
                )
            );

            let n = 4;
            let edges = [(1, 2)];
            let mut tree = TreeDistStrongly::new(n);
            for &(u, v) in &edges {
                tree.add_edge(u - 1, v - 1);
            }
            assert_eq!(
                tree.distance(0),
                (vec![Some(0), Some(1), None, None], vec![0, 0, 2, 3])
            )
        }

        #[test]
        fn for_count_subtree_strongly() {
            let nodes = 6;
            let edges = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
            let mut tree = TreeDistStrongly::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(tree.count_subtree_from(0), vec![6, 1, 1, 1, 1, 1]);

            let nodes = 12;
            let edges = vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (7, 9),
                (7, 10),
                (8, 11),
                (8, 12),
            ];
            let mut tree = TreeDistStrongly::new(nodes);
            for (u, v) in edges {
                tree.add_edge(u - 1, v - 1);
                tree.add_edge(v - 1, u - 1);
            }
            assert_eq!(
                tree.count_subtree_from(0),
                vec![12, 3, 4, 4, 1, 1, 3, 3, 1, 1, 1, 1]
            );
            assert_eq!(
                tree.count_subtree_from(6),
                vec![8, 3, 9, 4, 1, 1, 12, 3, 1, 1, 1, 1]
            );
        }

        #[test]
        fn for_strongly_connected_components() {
            let mut tree = TreeDistStrongly::new(7);
            let edges = vec![
                (0, 6),
                (1, 0),
                (2, 6),
                (2, 3),
                (3, 4),
                (4, 3),
                (5, 0),
                (5, 4),
                (6, 1),
                (6, 3),
                (6, 4),
                (6, 5),
            ];
            for (v, w) in edges {
                tree.add_edge(v, w);
            }
            assert_eq!(
                tree.strongly_connected_component(),
                vec![1, 1, 0, 2, 2, 1, 1]
            );
            // decide this graph can have topological ordering
            assert_eq!(
                {
                    let mut v = tree.strongly_connected_component();
                    v.sort();
                    v.iter().zip(v.iter().skip(1)).all(|(&a, &b)| a != b)
                },
                false
            );
            let mut tree = TreeDistStrongly::new(7);
            let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (4, 6), (5, 6)];
            for (v, w) in edges {
                tree.add_edge(v, w);
            }
            assert_eq!(
                tree.strongly_connected_component(),
                vec![0, 3, 1, 6, 4, 2, 5]
            );
            // decide this graph can have topological ordering
            assert_eq!(
                {
                    let mut v = tree.strongly_connected_component();
                    v.sort();
                    v.iter().zip(v.iter().skip(1)).all(|(&a, &b)| a != b)
                },
                true
            );
        }

        #[test]
        fn for_euler_walk_strongly() {
            let mut tree = TreeDistStrongly::new(5);
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 2),
                (2, 4),
                (4, 1),
                (1, 3),
                (3, 0),
            ];
            for &(v, w) in &edges {
                tree.add_edge(v, w);
            }
            let walk = tree.eulerian_walk();
            let mut edges = edges.into_iter().collect::<std::collections::HashSet<_>>();
            let mut past = walk.iter().last().unwrap().1;
            for &(v, w) in &walk {
                // connected
                assert_eq!(past, v);
                // edges contains walk's edge
                assert!(edges.remove(&(v, w)));
                past = w;
            }
            // all edges are used
            assert_eq!(edges.len(), 0);
        }

        #[test]
        fn for_euler_walk() {
            let mut tree = TreeDist::new(5);
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 2),
                (2, 4),
                (4, 1),
                (1, 3),
                (3, 0),
            ];
            for &(v, w) in &edges {
                tree.add_edge(v, w);
            }
            let walk = tree.eulerian_walk();
            let mut edges = edges.into_iter().collect::<std::collections::HashSet<_>>();
            let mut past = walk.iter().last().unwrap().1;
            for &(v, w) in &walk {
                // connected
                assert_eq!(past, v);
                // edges contains walk's edge
                assert!(edges.remove(&(v, w)));
                past = w;
            }
            // all edges are used
            assert_eq!(edges.len(), 0);
        }
    }
}
