#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        robots: [(i64, i64); n]
    }
    let mut robots = robots
        .into_iter()
        .map(|(x, l)| (x - l, x + l))
        .collect::<Vec<_>>();
    robots.sort_by_key(|e| e.1);
    let mut last = std::i64::MIN;
    let mut count = 0;
    for (s, t) in robots {
        if last <= s {
            last = t;
            count += 1;
        }
    }
    println!("{}", count);
}
