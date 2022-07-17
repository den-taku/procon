#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xys: [(i64, i64); n]
    }
    println!(
        "{}",
        std::cmp::max(
            xys.iter().map(|(x, y)| x + y).max().unwrap()
                - xys.iter().map(|(x, y)| x + y).min().unwrap(),
            xys.iter().map(|(x, y)| x - y).max().unwrap()
                - xys.iter().map(|(x, y)| x - y).min().unwrap(),
        )
    );
}
