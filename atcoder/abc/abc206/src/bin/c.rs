#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    a.sort();
    let mut ans = 0;
    for e in &a {
        let down = binary_search_lower(*e, &a);
        let up = binary_search_upper(*e, &a);
        if let Some(d) = down {
            if let Some(u) = up {
                ans += a.len() + 1 - u + d;
            } else {
                ans += d + 1;
            }
        } else {
            if let Some(u) = up {
                ans += a.len() - u;
            }
        }
    }
    println!("{}", ans);
}

#[inline(always)]
fn binary_search_lower(v: u64, a: &[u64]) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = a.len() - 1;
    while upper_bound - lower_bound > 1 {
        let est = (upper_bound + lower_bound) / 2;
        if a[est] < v {
            lower_bound = est;
        } else {
            upper_bound = est;
        }
    }
    if a[upper_bound] < v {
        Some(upper_bound)
    } else if a[lower_bound] < v {
        Some(lower_bound)
    } else {
        None
    }
}

#[inline(always)]
fn binary_search_upper(v: u64, a: &[u64]) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = a.len() - 1;
    while upper_bound - lower_bound > 1 {
        let est = (upper_bound + lower_bound) / 2;
        if a[est] > v {
            upper_bound = est;
        } else {
            lower_bound = est;
        }
    }
    if a[lower_bound] > v {
        Some(lower_bound)
    } else if a[upper_bound] < v {
        Some(upper_bound)
    } else {
        None
    }
}
