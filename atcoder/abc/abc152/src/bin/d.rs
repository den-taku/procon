#![allow(unreachable_code)]
#![allow(clippy::many_single_char_names)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0u64;
    let mut dp = [0u64; 9 * 9];
    // let (na, nb) = split(n);
    for i in 1..10 {
        for j in 1..10 {
            dp[(i - 1) * 9 + j - 1] = i_j(n, i, j);
            dp[(j - 1) * 9 + i - 1] = i_j(n, j, i);
            ans += dp[(i - 1) * 9 + j - 1] * dp[(j - 1) * 9 + i - 1];
        }
    }
    println!("{}", ans);
}

#[inline]
fn split(mut a: usize) -> (usize, usize) {
    let little = a % 10;
    while a / 10 >= 1 {
        a /= 10;
    }
    (a, little)
}

#[inline]
fn i_j(n: usize, i: usize, j: usize) -> u64 {
    let mut count = 0u64;
    for k in 1..n + 1 {
        let (a, b) = split(k);
        if a == i && b == j {
            count += 1;
        }
    }
    count
}
