#![allow(unreachable_code)]
#![allow(dead_code)]
use proconio::{fastout, input};

const MOD: u64 = 998244353;

// BIT and Compress
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut pows = vec![1; n];
    for i in 1..n {
        pows[i] = 2 * pows[i - 1] % MOD;
    }
    let mut v = vec![(a[0], 0)];
    let mut ans = vec![0; n];
    // ans[0] = 1;
    for (i, &e) in a.iter().enumerate().skip(1) {
        let k = binary_search(&v, e);
        if let Some(k) = k {
            println!("i: {}, k: {}", i, k);
            ans[i] += ans[k] * pows[i - k - 1] % MOD;
            ans[i] += ans[k] * pows[i - k - 1] % MOD;
            ans[i] += pows[i - k - 1] % MOD;
            ans[i] %= MOD;
        }
        v.push((e, i));
    }
    println!("{:?}", ans);
    println!("{}", ans.iter().fold(0, |s, &e| s + e % MOD));
}

fn binary_search(a: &[(usize, usize)], e: usize) -> Option<usize> {
    let mut ub = a.len() - 1;
    let mut lb = 0;
    while ub - lb > 1 {
        let est = (ub + lb) / 2;
        if a[est].0 >= e {
            ub = est
        } else {
            lb = est
        }
    }
    if a[ub].0 <= e {
        Some(ub)
    } else if a[lb].0 <= e {
        Some(lb)
    } else {
        None
    }
}
