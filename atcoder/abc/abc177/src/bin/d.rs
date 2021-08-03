#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        relations: [(usize, usize); m]
    }
    let mut uf = UnionFind::new(n);
    for &(x, y) in &relations {
        let x = x - 1;
        let y = y - 1;
        uf.unite(x, y);
    }
    let mut map = std::collections::HashMap::new();
    for e in 0..n {
        let represent = uf.find(e);
        if let Some(&e) = map.get(&represent) {
            map.insert(represent, e + 1);
        } else {
            map.insert(represent, 1);
        }
    }
    println!("{}", map.iter().max_by_key(|e| e.1).unwrap().1);
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: {
                let mut v = Vec::with_capacity(n);
                for i in 0..n {
                    v.push(i);
                }
                v
            },
            rank: vec![0; n],
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.find(self.par[x])
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
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

    fn _same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
