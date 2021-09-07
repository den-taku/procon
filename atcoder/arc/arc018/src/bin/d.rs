// need to consider e1(not contain), e2(not contain), and e3(contain)
#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, u64); m]
    }
    let mut kruskal = kruskal_library::Kruskal::new(n);
    for &(u, v, c) in &edges {
        kruskal.add_edge(u - 1, v - 1, c);
    }
    let mst = kruskal.minimum_spanning_tree().unwrap();
    let min_cost = mst.iter().map(|e| e.2).sum::<u64>();
    let map = mst
        .iter()
        .copied()
        .map(|e| ((e.0, e.1), e.2))
        .collect::<std::collections::HashMap<(usize, usize), u64>>();
    let mut adjacent = vec![Vec::new(); n];
    for &(u, v, _) in &mst {
        adjacent[u].push(v);
        adjacent[v].push(u);
    }
    let mut ans = 1;
    for &(u, v, c) in &edges {
        let u = u - 1;
        let v = v - 1;
        if !map.contains_key(&(u, v)) {
            let mut visited = vec![false; n];
            let circuit = find_circuit(u, v, &adjacent, &mut visited);
            let mut count = 1;
            for (a, b) in circuit.windows(2).map(|e| (e[0], e[1])) {
                if let Some(&cost) = map.get(&(a, b)) {
                    if cost == c {
                        count += 1;
                    }
                } else if let Some(&cost) = map.get(&(b, a)) {
                    if cost == c {
                        count += 1;
                    }
                }
            }
            ans *= count;
            ans %= MOD;
        }
    }
    println!("{} {}", min_cost, ans);
}

fn find_circuit(u: usize, v: usize, mst: &[Vec<usize>], visited: &mut [bool]) -> Vec<usize> {
    let mut cand = Vec::new();
    if u == v {
        return vec![v];
    }
    for &e in &mst[u] {
        if !visited[e] {
            visited[e] = true;
            cand = find_circuit(e, v, &mst, visited);
        }
        if cand.len() > 0 {
            cand.push(u);
            break;
        }
    }
    cand
}

pub mod kruskal_library {
    /// verified by this(https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5857344#1).
    pub struct Kruskal<T> {
        vertics: usize,
        edges: Vec<(usize, usize, T)>,
    }

    impl<T> Kruskal<T>
    where
        T: std::cmp::Ord + Copy,
    {
        pub fn new(vertics: usize) -> Self {
            Self {
                vertics,
                edges: Vec::new(),
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize, weight: T) {
            self.edges.push((u, v, weight));
        }

        pub fn minimum_spanning_tree(&mut self) -> Option<Vec<(usize, usize, T)>> {
            let mut uft = union_find_library::UnionFind::new(self.vertics);
            self.edges.sort_by_key(|e| e.2);
            let mut minimum_spanning_tree = Vec::with_capacity(self.vertics - 1);
            for &(u, v, w) in self.edges.iter() {
                if uft.same(u, v) {
                    continue;
                } else {
                    uft.unite(u, v);
                    minimum_spanning_tree.push((u, v, w))
                }
                if minimum_spanning_tree.len() == self.vertics - 1 {
                    break;
                }
            }
            if minimum_spanning_tree.len() == self.vertics - 1 {
                Some(minimum_spanning_tree)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests_kruskal {
        use super::*;

        #[test]
        fn for_kruskal() {
            let vertics = 6;
            let edges = [
                (0, 1, 1),
                (0, 2, 3),
                (1, 2, 1),
                (1, 3, 7),
                (2, 4, 1),
                (1, 4, 3),
                (3, 4, 1),
                (3, 5, 1),
                (4, 5, 6),
            ];
            let mut kraskal = Kruskal::new(vertics);
            for (u, v, w) in edges {
                kraskal.add_edge(u, v, w);
            }
            assert_eq!(
                kraskal
                    .minimum_spanning_tree()
                    .unwrap()
                    .iter()
                    .map(|e| e.2)
                    .sum::<usize>(),
                5
            );
            let vertics = 4;
            let edges = [
                (0, 1, 2),
                (1, 2, 1),
                (2, 3, 1),
                (3, 0, 1),
                (0, 2, 3),
                (1, 3, 5),
            ];
            let mut kraskal = Kruskal::new(vertics);
            for (u, v, w) in edges {
                kraskal.add_edge(u, v, w);
            }
            assert_eq!(
                kraskal
                    .minimum_spanning_tree()
                    .unwrap()
                    .iter()
                    .map(|e| e.2)
                    .sum::<usize>(),
                3
            );
        }
    }

    pub mod union_find_library {
        /// Union-Find Tree, that treats disjoint sets efficiently.
        /// verified by this(https://atcoder.jp/contests/atc001/submissions/24929276).
        pub struct UnionFind {
            par: Vec<usize>,
            rank: Vec<usize>,
        }

        impl UnionFind {
            #[inline(always)]
            /// Create a new Union-Find Tree contains n elements.
            /// At the first state, the elements are mutually disjoint.
            pub fn new(n: usize) -> Self {
                UnionFind {
                    par: (0..n).collect(),
                    rank: vec![0; n],
                }
            }

            #[inline(always)]
            /// Return representative representing set containing x.
            pub fn find(&mut self, x: usize) -> usize {
                if x >= self.par.len() {
                    panic!("out of bound.")
                }
                unsafe {
                    if *self.par.get_unchecked(x) == x {
                        x
                    } else {
                        let mut represent = x;
                        while {
                            represent = *self.par.get_unchecked(represent);
                            *self.par.get_unchecked(represent) != represent
                        } {}
                        *self.par.get_unchecked_mut(x) = represent;
                        represent
                    }
                }
            }

            #[inline(always)]
            /// Unite 2 sets, one containing x and the other containing y.
            pub fn unite(&mut self, x: usize, y: usize) -> bool {
                let x_par = self.find(x);
                let y_par = self.find(y);
                if x_par != y_par {
                    unsafe {
                        if *self.rank.get_unchecked(x_par) < *self.rank.get_unchecked(y_par) {
                            *self.par.get_unchecked_mut(x_par) = y_par;
                        } else {
                            *self.par.get_unchecked_mut(y_par) = x_par;
                            if *self.rank.get_unchecked(x_par) == *self.rank.get_unchecked(y_par) {
                                self.rank[x_par] += 1;
                            }
                        }
                    }
                }
                x_par != y_par
            }

            #[inline(always)]
            /// Decide whether set, containing x, contains y or not.
            pub fn same(&mut self, x: usize, y: usize) -> bool {
                self.find(x) == self.find(y)
            }

            #[inline(always)]
            /// Convert UnionFind to Vec\<Vec\<usize\>\>
            pub fn to_vec(&self) -> Vec<Vec<usize>> {
                let mut set = vec![Vec::new(); self.par.len()];
                for (i, &p) in self.par.iter().enumerate() {
                    unsafe {
                        set.get_unchecked_mut(p).push(i);
                    }
                }
                set.into_iter().filter(|s| !s.is_empty()).collect()
            }
        }

        #[cfg(test)]
        mod tests_union_find {
            use super::*;

            #[test]
            fn for_union_find() {
                let queries = [
                    (0, 1, 2),
                    (0, 3, 2),
                    (1, 1, 3),
                    (1, 1, 4),
                    (0, 2, 4),
                    (1, 4, 1),
                    (0, 4, 2),
                    (0, 0, 0),
                    (1, 0, 0),
                ];
                let ans = [true, false, true, true];
                let n = 8;
                let mut uf_tree = UnionFind::new(n);
                let mut index = 0;
                for (i, x, y) in queries {
                    if i == 0 {
                        uf_tree.unite(x, y);
                    } else {
                        assert_eq!(uf_tree.same(x, y), ans[index]);
                        index += 1;
                    }
                }
            }
        }
    }
}
