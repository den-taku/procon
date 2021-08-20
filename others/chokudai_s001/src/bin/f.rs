#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut count = 1;
    let mut max = a[0];
    for &e in a.iter().skip(1) {
        if e < max {
            // count += 1;
        } else {
            max = e;
            count += 1;
        }
    }
    println!("{}", count);
}
