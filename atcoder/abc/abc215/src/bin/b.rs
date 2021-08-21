#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut ans = 0u32;
    let mut k = 1u64;
    while k * 2 <= n {
        k *= 2;
        ans += 1;
    }
    println!("{}", ans)
}
