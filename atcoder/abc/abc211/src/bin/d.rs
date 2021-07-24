use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut loads: [(usize, usize); m]
    }
    let mut edges = vec![Vec::new(); n];
    for &(a, b) in &loads {
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }
    let mut dis = vec![std::u64::MAX; n];
    let mut cnt = vec![0; n];
    let mut visited_from = vec![std::collections::HashSet::new(); n];
    dis[0] = 0;
    cnt[0] = 1;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    while let Some(node) = queue.pop_front() {
        for &dir in &edges[node] {
            if dis[dir] == dis[node] + 1 && !visited_from[dir].contains(&node) {
                cnt[dir] = (cnt[dir] + cnt[node]) % MOD;
                visited_from[dir].insert(node);
                if dir != n - 1 {
                    queue.push_back(dir);
                }
            } else if dis[dir] > dis[node] + 1 {
                cnt[dir] = cnt[node];
                dis[dir] = dis[node] + 1;
                visited_from[dir] = [node]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<_>>();
                if dir != n - 1 {
                    queue.push_back(dir);
                }
            }
        }
        // println!("dis: {:?}", dis);
        // println!("queue: {:?}", queue);
        // println!("cnt: {:?}", cnt);
        // println!("");
    }
    println!("{}", cnt[n - 1]);
}

#[derive(Clone, Debug)]
struct Matrix {
    vec: Vec<u64>,
    len: usize,
}

impl Matrix {
    fn new(len: usize) -> Self {
        Self {
            vec: vec![0; len * len],
            len,
        }
    }

    fn add(&mut self, a: usize, b: usize) {
        self.vec[a * self.len + b] += 1;
        self.vec[b * self.len + a] += 1;
    }
}

impl Matrix {
    fn pow(self, rhs: &Self) -> Self {
        // let mut v = vec![0; self.len * self.len];
        // for i in 0..self.len {
        //     for j in 0..self.len {
        //         let mut sum = 0;
        //         for k in 0..self.len {
        //             sum += (self.vec[i * self.len + k] * rhs.vec[k * self.len + k]) % MOD;
        //             sum %= MOD;
        //         }
        //         v[i * self.len + j] = sum
        //     }
        // }
        let k = self.len;
        let mut array = vec![0; k * k];
        (0..k).fold((), |_, i| {
            (0..k).fold((), |_, j| {
                array[i * k + j] = (0..k).fold(0, |e, l| {
                    (e + ((self.vec[i * k + l] * rhs.vec[j + l * k]) % MOD)) % MOD
                })
            })
        });
        Self { vec: array, len: k }
        // Self {
        //     vec: v,
        //     len: self.len,
        // }
    }
}
