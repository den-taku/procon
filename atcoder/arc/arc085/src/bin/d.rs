#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        _z: i64,
        w: i64,
        a: [i64; n]
    }
    if n == 1 {
        println!("{}", (a[0] - w).abs())
    } else {
        println!(
            "{}",
            std::cmp::max((a[n - 1] - w).abs(), (a[n - 2] - a[n - 1]).abs())
        )
    }
}
