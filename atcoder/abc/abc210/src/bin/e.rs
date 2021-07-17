use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut dir: [(usize, i64); m]
    }
    dir.sort_by_key(|e| e.1);
    let mut uf = UnionFind::new(n);
    let mut cost = 0;
    let mut s = 1;
    for d in dir {
        for i in 0..n {
            if !uf.same(i, (i + d.0) % n) {
                cost += d.1;
                uf.unite(i, (i + d.0) % n);
                s += 1;
                if s == n {
                    println!("{}", cost);
                    return;
                }
            }
        }
    }
    println!("-1");
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind {
            par: vec,
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn unite(&mut self, a: usize, b: usize) {
        let apar = self.find(a);
        let bpar = self.find(b);
        if self.rank[apar] > self.rank[bpar] {
            self.par[bpar] = apar;
        } else {
            self.par[apar] = bpar;
            if self.rank[apar] == self.rank[bpar] {
                self.rank[bpar] += 1;
            }
        }
    }
}
