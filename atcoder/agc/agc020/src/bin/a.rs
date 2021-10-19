#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize
    }
    if (b - a - 1) % 2 == 0 {
        println!("Borys")
    } else {
        println!("Alice")
    }
}
