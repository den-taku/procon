#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_s: [u64; n],
        mut b_s: [u64; m]
    }
    b_s.sort();
    let mut ans = std::u64::MAX;
    for &a in &a_s {
        if let Some(e) = binary_search(a, &b_s) {
            ans = std::cmp::min(ans, e - a);
        }
    }
    for &a in &a_s {
        if let Some(e) = binary_search2(a, &b_s) {
            ans = std::cmp::min(ans, a - e);
        }
    }
    println!("{}", ans);
}

#[inline]
fn binary_search(a: u64, b_s: &[u64]) -> Option<u64> {
    let mut upperbound = b_s.len() - 1;
    let mut lowerbound = 0;
    while upperbound - lowerbound > 1 {
        let est = (lowerbound + upperbound) / 2;
        if condition(a, b_s[est]) {
            upperbound = est;
        } else {
            lowerbound = est;
        }
    }
    if condition(a, b_s[lowerbound]) {
        Some(b_s[lowerbound])
    } else if condition(a, b_s[upperbound]) {
        Some(b_s[upperbound])
    } else {
        None
    }
}

#[inline]
fn condition(a: u64, est: u64) -> bool {
    est >= a
}

#[inline]
fn binary_search2(a: u64, b_s: &[u64]) -> Option<u64> {
    let mut upperbound = b_s.len() - 1;
    let mut lowerbound = 0;
    while upperbound - lowerbound > 1 {
        let est = (lowerbound + upperbound) / 2;
        if condition2(a, b_s[est]) {
            lowerbound = est;
        } else {
            upperbound = est;
        }
    }
    if condition2(a, b_s[upperbound]) {
        Some(b_s[upperbound])
    } else if condition2(a, b_s[lowerbound]) {
        Some(b_s[lowerbound])
    } else {
        None
    }
}

#[inline]
fn condition2(a: u64, est: u64) -> bool {
    est <= a
}
