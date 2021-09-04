#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        l: usize,
        q: usize,
        queries: [(usize, usize); q]
    }
    let mut tree = BTreeSet::new();
    tree.insert(0);
    tree.insert(l);
    for (c, x) in queries {
        if c == 1 {
            tree.insert(x);
        } else {
            let l = tree.range(..x).next_back().unwrap();
            let u = tree.range(x..).next().unwrap();
            // println!("{} {}", u, l);
            println!("{}", u - l);
        }
    }
}
