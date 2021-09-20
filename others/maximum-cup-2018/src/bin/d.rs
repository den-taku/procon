#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        x: usize,
        a: [usize; n]
    }
    let mut dp = vec![None; m];
    dp[0] = Some(0);
    for e in a {
        let mut next = dp.clone();
        for i in 0..m {
            if let Some(c) = dp[i] {
                let over = (i + e) / m;
                if c + over >= x {
                    continue;
                }
                if let Some(v) = dp[(i + e) % m] {
                    next[(i + e) % m] = Some(std::cmp::min(v, c + over))
                } else {
                    next[(i + e) % m] = Some(c + over)
                }
            }
        }
        dp = next;
    }
    if dp[l].is_some() {
        println!("Yes")
    } else {
        println!("No")
    }
}
