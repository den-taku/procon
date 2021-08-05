#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [f64; n]
    }
    println!("{}", ternary_search(n, &a) / n as f64);
}

#[inline]
fn ternary_search(_n: usize, a: &[f64]) -> f64 {
    let mut upper_bound = a.iter().copied().fold(0.0, max) * 2.0;
    let mut lower_bound = 0.0;
    while (upper_bound - lower_bound).abs() >= 1e-6 {
        let u = 2.0 * upper_bound / 3.0 + lower_bound / 3.0;
        let l = upper_bound / 3.0 + 2.0 * lower_bound / 3.0;
        let fu = a
            .iter()
            .copied()
            .map(|e| u + e - min(e, 2.0 * u))
            .sum::<f64>();
        let fl = a
            .iter()
            .copied()
            .map(|e| l + e - min(e, 2.0 * l))
            .sum::<f64>();
        if fu > fl {
            upper_bound = u;
        } else {
            lower_bound = l;
        }
    }
    let fu = a
        .iter()
        .copied()
        .map(|e| upper_bound + e - min(e, 2.0 * upper_bound))
        .sum::<f64>();
    let fl = a
        .iter()
        .copied()
        .map(|e| lower_bound + e - min(e, 2.0 * lower_bound))
        .sum::<f64>();
    if fu > fl {
        fl
    } else {
        fu
    }
}

#[inline]
fn min(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

#[inline]
fn max(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}
