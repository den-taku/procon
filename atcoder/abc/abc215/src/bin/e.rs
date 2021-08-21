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
        for set in 1..2 << 10 {
            for last in 0..10 {
                if last == c {
                    next[set][last] += dp[set][last];
                    next[set][last] %= MOD;
                } else if set >> c & 1 == 0 {
                    next[set | 1 << c][c] += dp[set][last];
                    next[set | 1 << c][c] %= MOD;
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for set in 0..2 << 10 {
        for last in 0..10 {
            ans += dp[set][last];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
