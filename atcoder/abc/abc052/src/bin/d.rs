#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        xs: [usize; n]
    }
    println!(
        "{}",
        xs.windows(2)
            .map(|e| (e[0], e[1]))
            .map(|(s, t)| std::cmp::min(b, (t - s) * a))
            .sum::<usize>()
    )
}
