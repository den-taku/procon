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
    for b in s.clone() {
        if let Some(&v) = blue.get(&b) {
            blue.insert(b, v + 1);
        } else {
            blue.insert(b, 1);
        }
    }
    let mut red = HashMap::new();
    for r in t {
        if let Some(&v) = red.get(&r) {
            red.insert(r, v + 1);
        } else {
            red.insert(r, 1);
        }
    }
    println!(
        "{}",
        std::cmp::max(
            s.into_iter()
                .map(|b| { blue.get(&b).unwrap() - red.get(&b).unwrap_or(&0) })
                .max()
                .unwrap(),
            0
        )
    )
}
