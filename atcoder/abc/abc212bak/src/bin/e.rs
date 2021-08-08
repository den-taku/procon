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
        let sum = dp.iter().sum::<u64>();
        let mut next = dp.iter().map(|v| sum - v).collect::<Vec<_>>();
        for &(u, v) in &broken {
            let u = u - 1;
            let v = v - 1;
            unsafe {
                *next.get_unchecked_mut(u) -= *dp.get_unchecked(v);
                *next.get_unchecked_mut(v) -= *dp.get_unchecked(u);
            }
        }
        dp = next.iter().map(|v| v % MOD).collect::<Vec<_>>();
    }
    println!("{}", dp[0]);
}
