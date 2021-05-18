use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        _m: usize,
        f: [i64; n]
    }
    println!("{}", dynamic(n, f));
}

fn dynamic(n: usize, f: Vec<i64>) -> i64 {
    let mut dp = vec![0i64; n];
    let mut now = HashSet::new();
    dp[0] = 1;
    now.insert(f[0]);
    let mut sum = 1;
    for i in 1..n {
        if !now.insert(f[i]) {
            while f[i] != f[i - now.len()] {
                now.remove(&f[i - now.len()]);
            }
        }
        if now.len() == i + 1 {
            sum += dp[i - 1];
            dp[i] += sum;
        } else {
            for j in 0..now.len() {
                dp[i] += dp[i - 1 - j];
            }
        }
        dp[i] %= 1000000007;
    }
    dp.pop().unwrap()
}
