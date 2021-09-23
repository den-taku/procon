#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        m: usize,
        x: [isize; m]
    }
    let x = x.into_iter().map(|e| e - 1).collect::<Vec<_>>();
    let mut ub = 2 * n as isize;
    let mut lb = -1;
    while ub - lb > 1 {
        let est = (lb + ub) / 2;
        if condition(est, &x, n) {
            ub = est
        } else {
            lb = est
        }
    }
    println!("{}", ub)
}

#[inline(always)]
fn condition(est: isize, x: &[isize], n: isize) -> bool {
    let mut i = -1;
    for &e in x {
        if i >= e - 1 {
            i = e + est;
            continue;
        }
        let a = e - i - 1;
        if est < a {
            return false;
        }
        i = e + std::cmp::max(est - 2 * a, (est - a) / 2)
    }
    i >= n - 1
}
