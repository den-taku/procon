#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        hs: [(usize, usize); n]
    }
    let max = hs.iter().map(|(h, s)| h + s * hs.len()).max().unwrap();
    let mut ub = max + 3;
    let mut lb = 1;
    while ub - lb > 1 {
        let est = (ub + lb) / 2;
        if condition(est, &hs) {
            ub = est;
        } else {
            lb = est;
        }
    }
    if condition(lb, &hs) {
        println!("{}", lb)
    } else {
        println!("{}", ub)
    }
}

#[inline(always)]
fn condition(est: usize, hs: &[(usize, usize)]) -> bool {
    let mut v = vec![];
    for (h, s) in hs {
        if &est < h {
            return false;
        }
        v.push((est - h) / s)
    }
    v.sort();
    for (i, &e) in v.iter().enumerate() {
        if i > e {
            return false;
        }
    }
    true
}
