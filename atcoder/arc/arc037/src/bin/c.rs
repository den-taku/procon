#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ai: [usize; n],
        bj: [usize; n]
    }
    let mut heap = BinaryHeap::new();
    for &a in ai.iter() {
        for &b in bj.iter() {
            if heap.len() < k {
                heap.push(a * b);
            } else {
                if *heap.peek().unwrap() > a * b {
                    heap.pop();
                    heap.push(a * b);
                }
            }
        }
    }
    println!("{}", heap.peek().unwrap());
}
