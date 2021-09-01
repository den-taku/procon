#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    }
    let mut heap = ab
        .into_iter()
        .map(|e| Reverse(e))
        .collect::<BinaryHeap<_>>();
    println!(
        "{}",
        (0..k)
            .map(|_| {
                let Reverse(e) = heap.pop().unwrap();
                heap.push(Reverse((e.0 + e.1, e.1)));
                e.0
            })
            .sum::<usize>()
    )
}
