#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [u64; n]
    }
    println!("{}", aa.iter().sum::<u64>())
}
