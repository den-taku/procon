#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut sum = 0;
    for i in 0.. {
        sum += i;
        if sum >= n {
            println!("{}", i);
            return;
        }
    }
}
