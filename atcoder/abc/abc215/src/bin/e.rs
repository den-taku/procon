#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998244353;
const A: usize = b'A' as usize;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String
    }
    let mut dp = [[0u64; 10]; 2048];
    for c in s.chars().map(|c| c as usize - A) {
        let mut next = dp.clone();
        next[1 << c][c] += 1;
        let mut p;
        for set in 1..2 << 10 {
            for last in 0..10 {
                if last == c {
                    p = &mut next[set][last];
                    *p += dp[set][last];
                    *p %= MOD;
                } else if set >> c & 1 == 0 {
                    p = &mut next[set | 1 << c][c];
                    *p += dp[set][last];
                    *p %= MOD;
                }
            }
        }
        dp = next;
    }
    println!(
        "{}",
        dp.iter()
            .map(|v| v.iter().fold(0, |i, e| (i + e) % MOD))
            .fold(0, |i, e| (i + e) % MOD)
    )
}
