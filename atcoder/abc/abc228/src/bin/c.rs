#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n * 3]
    }
    let mut now = vec![0usize; n];
    for i in 0..n {
        for j in 0..3 {
            now[i] += p[i * 3 + j]
        }
    }
    let mut rank = now.clone();
    rank.sort();
    rank.reverse();
    let kth = rank[k - 1];
    // println!("{:?}", now);
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if now[i] + 300 >= kth {
            ans.push("Yes".to_string())
        } else {
            ans.push("No".to_string())
        }
    }
    println!("{}", ans.join("\n"))
}
