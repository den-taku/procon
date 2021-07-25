use proconio::{fastout, input};

const MOD: u64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ind: [(String, usize); k]
    }
    let mut set = std::collections::HashSet::new();
    let mut ls = Vec::new();
    let mut rs = Vec::new();
    for (i, (c, k)) in ind.iter().cloned().enumerate() {
        if c.starts_with("L") {
            ls.push((k - 1, i + 1));
        } else if c.starts_with("R") {
            rs.push((k - 1, i + 1));
            set.insert(i + 1);
        }
    }
    ls.sort();
    ls.reverse();
    rs.sort();
    rs.reverse();
    let mut ans = 1;
    for i in 0..n {
        if !ls.is_empty() && ls[ls.len() - 1].0 == i {
            let value = ls.pop().unwrap();
            set.insert(value.1);
        } else if !rs.is_empty() && rs[rs.len() - 1].0 == i {
            let value = rs.pop().unwrap();
            set.remove(&value.1);
        } else {
            ans *= (set.len() as u64) % MOD;
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
