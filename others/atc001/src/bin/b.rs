#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(u8, usize, usize); q]
    }
    let mut uft = union_find_library::UnionFind::new(n);
    for (i, x, y) in queries {
        if i == 0 {
            uft.unite(x, y)
        } else if uft.same(x, y) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

pub mod union_find_library {
    pub struct UnionFind {
        par: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        #[inline]
        pub fn new(n: usize) -> Self {
            UnionFind {
                par: (0..n).collect(),
                rank: vec![0; n],
            }
        }

        #[inline]
        pub fn find(&mut self, x: usize) -> usize {
            if x > self.par.len() {
                panic!("out out bound.")
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
                    *self.rank.get_unchecked_mut(x) = 1;
                    represent
                }
            }
        }

        #[inline]
        pub fn unite(&mut self, x: usize, y: usize) {
            if x > self.par.len() || y > self.par.len() {
                panic!("out out bound.")
            }
            let x_par = self.find(x);
            let y_par = self.find(y);
            if x_par == y_par {
                return;
            }
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

        #[inline]
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
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
                match i {
                    0 => uf_tree.unite(x, y),
                    _ => {
                        assert_eq!(uf_tree.same(x, y), ans[index]);
                        index += 1;
                    }
                }
            }
        }
    }
}
