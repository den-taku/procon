#![allow(unreachable_code)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut digit = Vec::new();
    while n > 0 {
        digit.push(n % 10);
        n /= 10;
    }
    let len = digit.len();
    let mut ans = 0;
    for v in digit.into_iter().permutations(len) {
        if v[0] == 0 {
            continue;
        }
        for i in 1..len {
            if v[i] == 0 {
                continue;
            }
            let mut a = 0;
            let mut d = 1;
            for j in (0..i).rev() {
                a += d * v[j];
                d *= 10;
            }
            let mut b = 0;
            let mut d = 1;
            for j in (i..len).rev() {
                b += d * v[j];
                d *= 10;
            }
            ans = std::cmp::max(ans, a * b);
        }
    }
    println!("{}", ans);
}
