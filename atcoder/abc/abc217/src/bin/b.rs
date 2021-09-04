#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }
    let slice = vec!["ABC", "ARC", "AGC", "AHC"]
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();
    let set = vec![s1, s2, s3]
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    for e in slice {
        if !set.contains(&e) {
            println!("{}", e);
            break;
        }
    }
}
