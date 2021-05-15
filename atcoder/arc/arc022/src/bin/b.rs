use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut best = HashSet::<i64>::new();
    let mut now = HashSet::<i64>::new();
    for i in 0..n {
        if !now.insert(a[i].clone()) {
            if now.len() > best.len() {
                best = now.clone();
            }
            while a[i - now.len()] != a[i] {
                now.remove(&a[i - now.len()]);
            }
        }
    }
    let ans = if best.len() > now.len() {
        best.len()
    } else {
        now.len()
    };
    println!("{}", ans);
}
