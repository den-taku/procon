#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n]
    }
    let mut dp = vec![None; (x + 1) * (y + 1)];
    dp[0] = Some(0);
    for (a, b) in ab.iter().map(|ab| (ab.0, ab.1)) {
        for i in (0..x + 1).rev() {
            for j in (0..y + 1).rev() {
                if let Some(c1) = dp[i * (y + 1) + j] {
                    if let Some(c2) = dp[min(x, i + a) * (y + 1) + min(y, j + b)] {
                        dp[min(x, i + a) * (y + 1) + min(y, j + b)] = Some(min(c1 + 1, c2));
                    } else {
                        dp[min(x, i + a) * (y + 1) + min(y, j + b)] = Some(c1 + 1)
                    }
                }
            }
        }
    }
    if let Some(c) = dp[x * (y + 1) + y] {
        println!("{}", c)
    } else {
        println!("-1")
    }
}
