#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
        x: u64
    }
    let sum = a.iter().sum::<u64>();
    let mut all = 0;
    let index = x / sum;
    all += index * sum;
    for (i, &e) in a.iter().enumerate() {
        all += e;
        if all > x {
            println!("{}", index * n as u64 + i as u64 + 1);
            return;
        }
    }
}
