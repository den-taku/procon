#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        mut sum: usize
    }
    let mut ans = sum;
    while sum >= m {
        let count = sum / m;
        ans += n * count;
        sum -= m * count;
        sum += n * count;
    }
    println!("{}", ans);
}
