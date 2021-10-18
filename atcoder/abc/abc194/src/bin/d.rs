#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0.0;
    for i in 1..n {
        ans += n as f64 / i as f64;
    }
    println!("{}", ans);
}
