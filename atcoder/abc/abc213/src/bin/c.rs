#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut ha = ab.iter().map(|e| e.0).collect::<Vec<_>>();
    ha.sort();
    ha.dedup();
    let mut wb = ab.iter().map(|e| e.1).collect::<Vec<_>>();
    wb.sort();
    wb.dedup();
    for &(a, b) in &ab {
        let ah = if let Some(v) = binary_search(&ha, a) {
            v
        } else {
            1
        };
        let bw = if let Some(v) = binary_search(&wb, b) {
            v
        } else {
            1
        };
        println!("{} {}", ah + 1, bw + 1);
    }
}

#[inline]
fn binary_search(a: &[usize], v: usize) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = std::cmp::min(v, a.len() - 1);
    if a[upper_bound] <= v {
        return Some(upper_bound);
    }
    while upper_bound - lower_bound > 1 {
        let est = (upper_bound + lower_bound) / 2;
        if a[est] <= v {
            lower_bound = est;
        } else {
            upper_bound = est;
        }
    }
    if a[upper_bound] <= v {
        Some(upper_bound)
    } else if a[lower_bound] <= v {
        Some(lower_bound)
    } else {
        None
    }
}
