#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut q = vec![0; n];
    for (i, e) in p.into_iter().enumerate() {
        q[e - 1] = i + 1;
    }
    println!(
        "{}",
        q.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
