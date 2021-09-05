use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m]
    }
    let mut blue = HashMap::new();
    for b in s {
        *blue.entry(b).or_insert(0) += 1;
    }
    let mut red = HashMap::new();
    for r in t {
        *red.entry(r).or_insert(0) += 1;
    }
    println!(
        "{}",
        std::cmp::max(
            blue.into_iter()
                .map(|(d, c)| { c - red.get(&d).unwrap_or(&0) })
                .max()
                .unwrap(),
            0
        )
    )
}
