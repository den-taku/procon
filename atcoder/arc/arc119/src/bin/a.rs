#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    let mut p = vec![0u64; 60];
    for i in 0..60 {
        p[i] = 2u64.pow(i as u32);
    }
    let mut min = n;
    for b in 1..60 {
        let a = n / p[b];
        let c = n % p[b];
        min = std::cmp::min(min, a + c + b as u64)
    }
    println!("{}", min)
}
