#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [f64; n]
    }
    let mut ans = vec![0; n];
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            ans[i] ^= 1;
            ans[i + 1] ^= 1;
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
