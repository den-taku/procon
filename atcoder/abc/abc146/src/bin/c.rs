#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }
    println!("{}", binary_search(a, b, x))
}

fn binary_search(a: u64, b: u64, x: u64) -> u64 {
    let mut min = 0;
    let mut max = 1_000_000_000;
    while max - min > 1 {
        let est = (max + min) / 2;
        if cond(est, a, b, x) {
            min = est
        } else {
            max = est
        }
    }
    if cond(max, a, b, x) {
        max
    } else {
        min
    }
}

fn cond(est: u64, a: u64, b: u64, x: u64) -> bool {
    let d = {
        let mut x = est;
        let mut d = 0;
        while x > 0 {
            d += 1;
            x /= 10;
        }
        d
    };
    a * est + b * d <= x
}
