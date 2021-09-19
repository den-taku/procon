#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        mut edge: [(usize, usize); m]
    }
    edge.sort_by(|a, b| if a.0 == b.0 { b.1.cmp(&a.1) } else { a.0.cmp(&b.0) });
    let mut dp = vec![edge[0].1];
    for &(_, b) in edge.iter().skip(1) {
        if b > dp[dp.len() - 1] {
            dp.push(b)
        } else {
            let mut ub = dp.len() - 1;
            let mut lb = 0;
            while ub - lb > 1 {
                let est = (ub + lb) / 2;
                if dp[est] < b {
                    lb = est
                } else {
                    ub = est
                }
            }
            if dp[lb] < b {
                dp[ub] = b
            } else {
                dp[lb] = b
            }
        }
    }
    println!("{}", dp.len())
}
