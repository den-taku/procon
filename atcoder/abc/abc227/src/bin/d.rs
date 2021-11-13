#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut ub = a.iter().sum::<usize>() + 2;
    ub *= k;
    let mut lb = 0;
    while ub - lb > 1 {
        let est = (ub + lb) / 2;
        if condition(est, &a, k) {
            lb = est
        } else {
            ub = est
        }
    }
    if condition(ub, &a, k) {
        println!("{}", ub)
    } else {
        println!("{}", lb)
    }
}

fn condition(est: usize, a: &[usize], k: usize) -> bool {
    a.iter().map(|&e| std::cmp::min(e, est)).sum::<usize>() >= est * k
}
