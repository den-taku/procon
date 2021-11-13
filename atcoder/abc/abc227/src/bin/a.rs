#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize
    }
    println!("{}", (k + a - 2) % n + 1)
}
