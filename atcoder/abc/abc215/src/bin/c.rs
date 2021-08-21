#![allow(unreachable_code)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        k: usize
    }
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    let mut ss = s.iter().permutations(s.len()).collect::<Vec<_>>();
    ss.sort();
    ss.dedup();
    for ans in ss.iter().skip(k - 1).take(1) {
        for c in ans {
            print!("{}", c);
        }
    }
    println!("")
}
