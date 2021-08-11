#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    }
    let mut dp = vec![0usize; (n + 1) * (k + 1) * (w + 1)];
    for a in 1..w + 1 {
        for b in 1..k + 1 {
            for c in 1..n + 1 {
                unsafe {
                    if a >= ab.get_unchecked(c - 1).0 {
                        *dp.get_unchecked_mut(a * (k + 1) * (n + 1) + b * (n + 1) + c) = max(
                            *dp.get_unchecked(a * (k + 1) * (n + 1) + b * (n + 1) + c - 1),
                            *dp.get_unchecked(
                                (a - ab[c - 1].0) * (k + 1) * (n + 1) + (b - 1) * (n + 1) + c - 1,
                            ) + ab.get_unchecked(c - 1).1,
                        )
                    } else {
                        *dp.get_unchecked_mut(a * (k + 1) * (n + 1) + b * (n + 1) + c) =
                            *dp.get_unchecked(a * (k + 1) * (n + 1) + b * (n + 1) + c - 1);
                    }
                }
            }
        }
    }
    println!("{}", dp[w * (k + 1) * (n + 1) + k * (n + 1) + n]);
}
