#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        points: [usize; n]
    }
    let max: usize = points.iter().sum();
    let mut dp = vec![true; (n + 1) * (max + 1)];
    let mut count = 1;
    for i in 1..max + 1 {
        let mut valid = false;
        dp[i * (n + 1)] = false;
        for (j, &p) in points.iter().enumerate() {
            if valid || (i >= p && dp[(i - p) * (n + 1) + j]) {
                valid = true;
                break;
            } else {
                dp[i * (n + 1) + j + 1] = false;
            }
        }
        if valid {
            count += 1;
        }
    }
    println!("{}", count)
}
