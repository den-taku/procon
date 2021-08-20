#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    a.sort();
    println!(
        "{}",
        a.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
