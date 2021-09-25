#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n]
    }
    let mut ans = 0;
    for i in 1..n {
        if a[i - 1] + a[i] > x {
            ans += a[i - 1] + a[i] - x;
            a[i] -= std::cmp::min(a[i - 1] + a[i] - x, a[i]);
        }
    }
    println!("{}", ans);
}
