#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        wp: [(f64, f64); n]
    }
    let mut ub = 100.0;
    let mut lb = 0.0;
    while ub - lb > 1e-5 {
        let est = (ub + lb) / 2.0;
        if condition(&wp, est, k) {
            lb = est
        } else {
            ub = est
        }
    }
    if condition(&wp, ub, k) {
        println!("{}", ub)
    } else {
        println!("{}", lb)
    }
}

#[inline(always)]
fn condition(wp: &[(f64, f64)], est: f64, k: usize) -> bool {
    let mut array = wp.iter().map(|&(w, p)| w * p - est * w).collect::<Vec<_>>();
    array.sort_by(|a, b| b.partial_cmp(a).unwrap());
    array.iter().take(k).sum::<f64>() > 0.0
}
