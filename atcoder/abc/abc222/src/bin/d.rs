#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [usize; n],
        b_s: [usize; n]
    }
    let mut dp = [0u64; 3001];
    dp[0] = 1;
    for (&a, b) in a_s.iter().zip(b_s) {
        let mut next = [0; 3001];
        let mut sum = dp.iter().take(a).fold(0, |s, e| (s + e) % MOD);
        for i in a..b + 1 {
            sum += dp[i];
            sum %= MOD;
            next[i] = sum;
        }
        dp = next;
    }
    println!("{}", dp.iter().fold(0, |s, e| (s + e) % MOD));
}
