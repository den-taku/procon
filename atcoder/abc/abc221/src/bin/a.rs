#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32
    }
    println!("{}", 32u64.pow(a - b))
}
