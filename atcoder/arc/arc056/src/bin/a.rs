use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        k: u64,
        l: u64
    }
    let all1 = a * k;
    let allk = (k / l) * b + if k % l != 0 { b } else { 0 };
    let k1 = (k / l) * b + (k % l) * a;
    println!("{}", min(all1, min(allk, k1)))
}
