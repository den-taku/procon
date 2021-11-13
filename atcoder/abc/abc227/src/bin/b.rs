#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n]
    }
    let mut set = std::collections::HashSet::new();
    for a in 1..1000 {
        for b in 1..1000 {
            set.insert(4 * a * b + 3 * a + 3 * b);
        }
    }
    println!("{}", s.iter().filter(|s| !set.contains(s)).count())
}
