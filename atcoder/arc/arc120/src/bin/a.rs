#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       n: usize,
       a: [u64; n]
    }
    for i in 1..n + 1 {
        let mut b = a.clone();
        for j in 0..i {
            b[j] += b.iter().copied().take(i).max().unwrap();
        }
        println!("{}", b.iter().take(i).sum::<u64>())
    }
}
