#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize
    }
    if k < 13 {
        println!("{}", k);
        return;
    }
    let mut dp = vec![vec![vec![]; 10]; 10];
    for i in 0..=9 {
        dp[0][i].push(i);
    }
    let mut count = 9;
    let mut digit = 10;
    for i in 2..11 {
        for j in 0..=9 {
            if j == 0 {
                dp[i - 1][0] = dp[i - 2][0].clone();
                for &e in &dp[i - 2][1].clone() {
                    dp[i - 1][0].push(e)
                }
                continue;
            }
            for &e in &dp[i - 2][j - 1].clone() {
                let value = digit * j + e;
                count += 1;
                if count == k {
                    println!("{}", value);
                    return;
                } else {
                    dp[i - 1][j].push(value)
                }
            }
            for &e in &dp[i - 2][j].clone() {
                let value = digit * j + e;
                count += 1;
                if count == k {
                    println!("{}", value);
                    return;
                } else {
                    dp[i - 1][j].push(value)
                }
            }
            if j != 9 {
                for &e in &dp[i - 2][j + 1].clone() {
                    let value = digit * j + e;
                    count += 1;
                    if count == k {
                        println!("{}", value);
                        return;
                    } else {
                        dp[i - 1][j].push(value)
                    }
                }
            }
        }
        digit *= 10;
    }
}
