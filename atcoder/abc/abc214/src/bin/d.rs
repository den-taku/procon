#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut edges: [(usize, usize, u64); n - 1]
    }
    edges.sort_by_key(|e| e.2);
    let mut adjacent = vec![HashSet::new(); n];
    for &(a, b, _) in &edges {
        adjacent[a - 1].insert(b - 1);
        adjacent[b - 1].insert(a - 1);
    }
    let mut ans = 0;
    for &(a, b, w) in edges.iter().rev() {
        ans += w * adjacent[a - 1].len() as u64 * adjacent[b - 1].len() as u64;
        adjacent[a - 1].remove(&(b - 1));
        adjacent[b - 1].remove(&(a - 1));
    }
    println!("{}", ans);
    unimplemented!()
}
