#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        broken: [(usize, usize); m]
    }
    let mut dp = vec![0u64; n];
    dp[0] = 1;
    for _j in 1..k + 1 {
        let mut next = vec![0u64; n];
        let sum = dp.iter().sum::<u64>();
        for i in 0..n {
            next[i] = sum - dp[i];
        }
        for &(u, v) in &broken {
            let u = u - 1;
            let v = v - 1;
            next[u] -= dp[v];
            next[v] -= dp[u];
        }
        for ne in next.iter_mut().take(n) {
            *ne %= MOD;
        }
        dp = next;
    }
    println!("{}", dp[0]);
}
