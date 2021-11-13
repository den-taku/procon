#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    let mut ans = 0i64;
    for a in 1..n + 1 {
        if a * a * a > n {
            break;
        }
        for b in a..n + 1 {
            let c = n / (a * b);
            if c < b {
                break;
            }
            ans += c - b + 1
        }
    }
    println!("{}", ans);
}
