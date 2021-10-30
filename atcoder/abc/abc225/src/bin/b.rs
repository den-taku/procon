#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }
    let mut adjacent = vec![Vec::new(); n];
    for (a, b) in ab {
        adjacent[a - 1].push(b - 1);
        adjacent[b - 1].push(a - 1);
    }
    if adjacent.iter().any(|v| v.len() == n - 1) {
        println!("Yes")
    } else {
        println!("No")
    }
}
