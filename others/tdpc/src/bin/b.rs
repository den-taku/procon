#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        a: [usize; a],
        b: [usize; b]
    }
    let mut dp = vec![None; (a.len() + 1) * (b.len() + 1)];
    println!("{}", rec(0, 0, &mut dp, &a, &b).0);
}

fn rec(
    i: usize,
    j: usize,
    dp: &mut [Option<(usize, usize)>],
    a: &[usize],
    b: &[usize],
) -> (usize, usize) {
    if let Some(value) = dp[i * (b.len() + 1) + j] {
        value
    } else if i == a.len() && j == b.len() {
        (0, 0)
    } else if i == a.len() {
        let right = rec(i, j + 1, dp, a, b);
        let sum = right.1 + b[j];
        let hand = right.1 - right.0 + b[j];
        dp[i * (b.len() + 1) + j] = Some((hand, sum));
        (hand, sum)
    } else if j == b.len() {
        let left = rec(i + 1, j, dp, a, b);
        let sum = left.1 + a[i];
        let hand = left.1 - left.0 + a[i];
        dp[i * (b.len() + 1) + j] = Some((hand, sum));
        (hand, sum)
    } else {
        let left = rec(i + 1, j, dp, a, b);
        let sum_l = left.1 + a[i];
        let hand_l = left.1 - left.0 + a[i];
        let right = rec(i, j + 1, dp, a, b);
        let sum_r = right.1 + b[j];
        let hand_r = right.1 - right.0 + b[j];
        if hand_l > hand_r {
            dp[i * (b.len() + 1) + j] = Some((hand_l, sum_l));
            (hand_l, sum_l)
        } else {
            dp[i * (b.len() + 1) + j] = Some((hand_r, sum_r));
            (hand_r, sum_r)
        }
    }
}
