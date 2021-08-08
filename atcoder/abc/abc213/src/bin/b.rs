#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    let mut a = a
        .into_iter()
        .enumerate()
        .map(|e| (e.1, e.0))
        .collect::<Vec<_>>();
    a.sort();
    println!("{}", a[a.len() - 2].1 + 1);
}
