#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    for e in a..b + 1 {
        if e % c == 0 {
            println!("{}", e);
            return;
        }
    }
    println!("-1")
}
