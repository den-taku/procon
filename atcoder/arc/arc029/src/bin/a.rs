use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        ts: [i32; n]
    }
    if n <= 2 {
        println!("{}", ts.iter().max().unwrap());
    } else {
        let sum = ts.iter().sum::<i32>();
        let mut est = std::i32::MAX;
        for i in 0..n {
            est = min(est, max(sum - ts[i], ts[i]));
        }
        for v in (0..n).into_iter().permutations(2) {
            let a = ts[v[0]] + ts[v[1]];
            est = min(est, max(a, sum - a))
        }
        println!("{}", est);
    }
}
