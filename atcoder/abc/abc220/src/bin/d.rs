#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut dp = vec![0; 10];
    dp[a[0]] = 1;
    for e in a.into_iter().skip(1) {
        let mut next = vec![0; 10];
        for (i, &s) in dp.iter().enumerate() {
            next[(i * e) % 10] += s;
            next[(i * e) % 10] %= MOD;
            next[(i + e) % 10] += s;
            next[(i + e) % 10] %= MOD;
        }
        dp = next;
    }
    for e in dp {
        println!("{}", e);
    }
}
