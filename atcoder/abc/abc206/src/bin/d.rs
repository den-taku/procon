#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut uf = union_find_library::UnionFind::new(*a.iter().max().unwrap());
    for i in 0..n / 2 {
        uf.unite(a[i] - 1, a[a.len() - 1 - i] - 1);
    }
    println!("{}", uf.to_vec().iter().map(|v| v.len() - 1).sum::<usize>());
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
        pub fn to_vec(&mut self) -> Vec<Vec<usize>> {
            let mut set = vec![Vec::new(); self.par.len()];
            for i in 0..self.par.len() {
                unsafe {
                    set.get_unchecked_mut(self.find(i)).push(i);
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
