use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        costs: [i32; n]
    }
    let set = costs.iter().cloned().collect::<BTreeSet<i32>>();
    for e in set.iter().cloned().rev().skip(1).take(1) {
        println!("{}", e);
    }
}
