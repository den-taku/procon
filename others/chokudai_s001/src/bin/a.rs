#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    println!("{}", a.iter().max().unwrap())
}
