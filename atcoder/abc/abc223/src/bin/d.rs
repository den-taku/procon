#![allow(unreachable_code)]
#![allow(dead_code)]
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut adjacent = vec![Vec::new(); n];
    let mut indeg = vec![0; n];
    for (a, b) in ab {
        adjacent[a - 1].push(b - 1);
        indeg[b - 1] += 1;
    }
    let mut queue = std::collections::BinaryHeap::new();
    for i in 0..n {
        if indeg[i] == 0 {
            queue.push(Reverse(i));
        }
    }
    let mut ans = Vec::new();
    while let Some(Reverse(c)) = queue.pop() {
        for &x in &adjacent[c] {
            indeg[x] -= 1;
            if indeg[x] == 0 {
                queue.push(Reverse(x));
            }
        }
        ans.push((c + 1).to_string());
    }
    if ans.len() != n {
        println!("-1")
    } else {
        println!("{}", ans.join(" "))
    }
}
