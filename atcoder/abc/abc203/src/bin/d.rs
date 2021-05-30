use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n * n]
    }
    let midian = k * k / 2 + 1;
    let mut ans = std::i32::MAX;
    for i in 0..(n - k + 1) {
        for j in 0..(n - k + 1) {
            let mut set = Vec::with_capacity((n - k + 1) * (n - k + 1));
            for i_i in 0..k {
                for j_j in 0..k {
                    set.push(a[i * n + j + i_i * n + j_j]);
                }
            }
            set.sort_by_key(|&e| Reverse(e));
            let est = set[midian - 1];
            if ans > est {
                ans = est;
            }
        }
    }
    println!("{}", ans);
}
