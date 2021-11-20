#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: usize = 1048576;

#[fastout]
fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q]
    }
    let mut map: Vec<(Option<usize>, usize)> = (0..MOD + 3).map(|e| (None, e)).collect::<Vec<_>>();
    let mut ans = Vec::with_capacity(q);
    for (t, x) in tx {
        if t == 2 {
            if let Some(v) = map[x % MOD].0 {
                ans.push(v.to_string())
            } else {
                ans.push("-1".to_string())
            }
        } else {
            let (key, par) = find(x % MOD, &mut map);
            map[key] = (Some(x), par);
        }
    }
    println!("{}", ans.join("\n"));
}

fn find(x: usize, map: &mut [(Option<usize>, usize)]) -> (usize, usize) {
    if let Some(value) = map[x].0 {
        let (n_key, n_par) = find(map[x].1, map);
        map[x] = (Some(value), n_par);
        (n_key, n_par)
    } else {
        (x, (x + 1) % MOD)
    }
}
