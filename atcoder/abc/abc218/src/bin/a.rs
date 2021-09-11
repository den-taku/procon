#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }
    if s.chars().skip(n - 1).next().unwrap() == 'o' {
        println!("Yes")
    } else {
        println!("No")
    }
}
