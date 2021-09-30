#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h]
    }
    let mut dp = vec![None; h * w];
    for (i, v) in s.iter().enumerate() {
        for (j, c) in v.chars().enumerate() {
            if c == '.' {
                dp[i * w + j] = Some(1u64)
            }
        }
    }
    let mut left = vec![0u64; h * w];
    let mut under = vec![0u64; h * w];
    let mut naname = vec![0u64; h * w];
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            let mut sum = 0;
            if dp[i * w + j].is_none() {
                continue;
            }
            if i > 0 {
                if let Some(s) = dp[(i - 1) * w + j] {
                    let c = under[(i - 1) * w + j];
                    sum += s + c;
                    sum %= MOD;
                    under[i * w + j] = (c + s) % MOD;
                }
                if j > 0 {
                    if let Some(s) = dp[(i - 1) * w + j - 1] {
                        let c = naname[(i - 1) * w + j - 1];
                        sum += s + c;
                        sum %= MOD;
                        naname[i * w + j] = (c + s) % MOD;
                    }
                }
            }
            if j > 0 {
                if let Some(s) = dp[i * w + j - 1] {
                    let c = left[i * w + j - 1];
                    sum += s + c;
                    sum %= MOD;
                    left[i * w + j] = (c + s) % MOD;
                }
            }
            dp[i * w + j] = Some(sum);
        }
    }
    println!("{}", dp[h * w - 1].unwrap())
}
