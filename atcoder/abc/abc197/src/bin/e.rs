#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        xc: [(i64, usize); n]
    }
    let mut left = vec![std::i64::MAX; n + 2];
    let mut right = vec![std::i64::MIN; n + 2];
    let mut count = vec![0; n + 2];
    left[0] = 0;
    right[0] = 0;
    for (x, c) in xc {
        count[c] += 1;
        left[c] = min(left[c], x);
        right[c] = max(right[c], x);
    }
    count[n + 1] = 1;
    left[n + 1] = 0;
    right[n + 1] = 0;
    let mut l = 0;
    let mut r = 0;
    let mut now_index = 0;
    for i in 1..n + 2 {
        if count[i] == 0 {
            continue;
        }
        let dis = right[i] - left[i];
        let n_l = min(
            l + (right[i] - left[now_index]).abs(),
            r + (right[i] - right[now_index]).abs(),
        ) + dis;
        let n_r = min(
            l + (left[i] - left[now_index]).abs(),
            r + (left[i] - right[now_index]).abs(),
        ) + dis;
        now_index = i;
        l = n_l;
        r = n_r;
    }
    println!("{}", l);
}
