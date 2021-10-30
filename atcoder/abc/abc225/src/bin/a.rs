#![allow(unreachable_code)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut s = s.chars().collect::<Vec<_>>();
    let mut v = Vec::new();
    for s in s.iter().permutations(3) {
        v.push(s)
    }
    v.sort();
    v.dedup();
    println!("{}", v.len())
}
