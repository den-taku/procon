// Minimum Spanning Tree(https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A)
#![allow(unreachable_code)]
#![allow(dead_code)]

fn main() {
    let (v, edges) = input();
    let mut kruskal = kruskal_library::Kruskal::new(v);
    for (u, v, w) in edges {
        kruskal.add_edge(u, v, w);
    }
    let minimum_spanning_tree = kruskal.minimum_spanning_tree().unwrap();
    println!(
        "{}",
        minimum_spanning_tree.iter().map(|e| e.2).sum::<usize>()
    )
}

#[inline]
fn input() -> (usize, Vec<(usize, usize, usize)>) {
    let (v, e) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    let mut edges = Vec::with_capacity(e);
    for _ in 0..e {
        let e = read_line::<usize>();
        edges.push((e[0], e[1], e[2]));
    }
    (v, edges)
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

pub mod kruskal_library {
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
            let mut iter = self.edges.iter();
            while let Some(&(u, v, w)) = iter.next() {
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

    mod union_find_library {
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
            pub fn to_vec(&mut self) -> Vec<Vec<usize>> {
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
