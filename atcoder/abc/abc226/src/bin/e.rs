#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    }
    if n != m {
        println!("0");
        return;
    }
    let mut cond = vec![false; n];
    let mut tree = union_find_library::UnionFind::new(n);
    for (u, v) in uv {
        if cond[tree.find(u - 1)] && cond[tree.find(v - 1)] {
            println!("0");
            return;
        }
        if tree.same(u - 1, v - 1) {
            cond[tree.find(u - 1)] = true;
        } else {
            let condition = cond[tree.find(u - 1)] || cond[tree.find(v - 1)];
            tree.unite(u - 1, v - 1);
            cond[tree.find(u - 1)] = condition;
        }
    }
    println!("{}", pow(2, tree.to_vec().len()))
}

fn pow(a: u64, x: usize) -> u64 {
    if x == 0 {
        1
    } else if x == 1 {
        a % MOD
    } else if x % 2 == 0 {
        let p = pow(a, x / 2);
        p % MOD * p % MOD
    } else {
        let p = pow(a, x / 2);
        p % MOD * p % MOD * a % MOD
    }
}

/// Disjoint Set
///
/// new
/// unite
/// find
/// same
/// count
/// to_vec
pub mod union_find_library {
    /// Union-Find Tree, that treats disjoint sets efficiently.
    /// verified by this(https://atcoder.jp/contests/atc001/submissions/24929276).
    /// and (https://atcoder.jp/contests/abc214/submissions/26399785)
    pub struct UnionFind {
        par: Vec<usize>,
        rank: Vec<usize>,
        count: Vec<usize>,
    }

    impl UnionFind {
        #[inline(always)]
        /// Create a new Union-Find Tree contains n elements.
        /// At the first state, the elements are mutually disjoint.
        pub fn new(n: usize) -> Self {
            UnionFind {
                par: (0..n).collect(),
                rank: vec![0; n],
                count: vec![1; n],
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
                        *self.count.get_unchecked_mut(y_par) += *self.count.get_unchecked(x_par);
                    } else {
                        *self.par.get_unchecked_mut(y_par) = x_par;
                        *self.count.get_unchecked_mut(x_par) += *self.count.get_unchecked(y_par);
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

        #[inline(always)]
        /// count connected component's size of x
        pub fn count(&mut self, x: usize) -> usize {
            let x_par = self.find(x);
            unsafe { *self.count.get_unchecked(x_par) }
        }
    }

    #[cfg(test)]
    mod tests_union_find {
        use super::*;

        #[test]
        fn for_union_find() {
            let queries = vec![
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
            let ans = vec![true, false, true, true];
            let n = 8;
            let mut uf_tree = UnionFind::new(n);
            let mut index = 0;
            for &(i, x, y) in queries.iter() {
                if i == 0 {
                    uf_tree.unite(x, y);
                } else {
                    assert_eq!(uf_tree.same(x, y), ans[index]);
                    index += 1;
                }
            }
        }

        #[test]
        fn for_count_uftree() {
            let n = 5;
            let edges = vec![(1, 2, 1), (2, 3, 2), (4, 2, 5), (3, 5, 14)];
            let mut tree = UnionFind::new(n);
            let mut value = 0;
            for (u, v, w) in edges {
                value += w * tree.count(u - 1) * tree.count(v - 1);
                tree.unite(u - 1, v - 1);
            }
            assert_eq!(value, 76);
        }
    }
}
