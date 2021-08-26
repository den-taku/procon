#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    if n < 191 {
        println!("Yay!")
    } else if n == 191 {
        println!("so-so")
    } else {
        println!(":(")
    }
}
