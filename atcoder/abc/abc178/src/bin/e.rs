#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        xys: [(i64, i64); n]
    }
    let mut max_sum = std::i64::MIN;
    let mut min_sum = std::i64::MAX;
    let mut max_dif = std::i64::MIN;
    let mut min_dif = std::i64::MAX;

    for (x, y) in xys {
        let sum = x + y;
        let dif = x - y;
        max_sum = max(max_sum, sum);
        min_sum = min(min_sum, sum);
        max_dif = max(max_dif, dif);
        min_dif = min(min_dif, dif);
    }
    println!("{}", max(max_sum - min_sum, max_dif - min_dif))
}
