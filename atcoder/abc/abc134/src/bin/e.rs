#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut tree = BTreeSet::new();
    for (i, e) in a.iter().enumerate() {
        if let Some(&(v, j)) = tree.range(..(e, n - i)).next_back() {
            tree.remove(&(v, j));
            tree.insert((e, n - i));
        } else {
            tree.insert((e, n - i));
        }
    }
    println!("{}", tree.len());
}
