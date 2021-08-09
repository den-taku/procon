#![allow(dead_code)]

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
            let mut represent = x;
            while self.par[represent] != represent {
                represent = self.par[represent];
            }
            self.par[x] = represent;
            represent
        }

        #[inline]
        pub fn unite(&mut self, x: usize, y: usize) {
            let x_par = self.find(x);
            let y_par = self.find(y);
            if x_par == y_par {
                return;
            }
            if self.rank[x_par] < self.rank[y_par] {
                self.par[x_par] = y_par;
            } else {
                self.par[y_par] = x_par;
                if self.rank[x_par] == self.rank[y_par] {
                    self.rank[x_par] += 1;
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
            let query = [
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
            for (i, x, y) in query {
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
