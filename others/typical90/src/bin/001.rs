#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        cuts: [usize; n]
    }
    println!("{}", binary_search(k, l, &cuts));
}

#[inline]
fn binary_search(k: usize, l: usize, cuts: &[usize]) -> usize {
    let mut upper_bound = l / k;
    let mut lower_bound = 0;
    while upper_bound - lower_bound > 1 {
        let est = (upper_bound + lower_bound) / 2;
        if condition(k, l, est, cuts) {
            lower_bound = est;
        } else {
            upper_bound = est;
        }
    }
    if condition(k, l, upper_bound, cuts) {
        upper_bound
    } else {
        lower_bound
    }
}

#[inline]
fn condition(k: usize, l: usize, est: usize, cuts: &[usize]) -> bool {
    let mut last = 0;
    let mut index = 0;
    'out: for _ in 0..k {
        for i in index..cuts.len() {
            if cuts[i] - last >= est {
                last = cuts[i];
                index = i;
                continue 'out;
            }
        }
        return false;
    }
    l - cuts[index] >= est
}
