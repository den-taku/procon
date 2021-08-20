#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut last = Vec::new();
    last.push(a[0]);
    for e in a.into_iter().skip(1) {
        if last[last.len() - 1] < e {
            last.push(e);
        } else {
            let mut lower_bound = 0;
            let mut upper_bound = last.len() - 1;
            while upper_bound - lower_bound > 1 {
                let est = (upper_bound + lower_bound) / 2;
                if e < last[est] {
                    upper_bound = est;
                } else {
                    lower_bound = est;
                }
            }
            if e < last[lower_bound] {
                last[lower_bound] = e;
            } else {
                last[upper_bound] = e;
            }
        }
    }
    println!("{}", last.len());
}
