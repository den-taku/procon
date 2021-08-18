#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
    }
    println!(
        "{}",
        if n <= 125 {
            4
        } else if n <= 211 {
            6
        } else {
            8
        }
    )
}
