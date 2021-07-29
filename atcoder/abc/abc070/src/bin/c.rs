#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut t: [u128; n]
    }
    t.sort();
    t.dedup();
    let mut ans = 1;
    for e in t {
        ans = lcm(ans, e);
    }
    println!("{}", ans);
}

#[inline]
fn gcd(a: u128, b: u128) -> u128 {
    let mut max = std::cmp::max(a, b);
    let mut min = std::cmp::min(a, b);
    while min != 0 {
        let tmp = min;
        min = max % min;
        max = tmp;
    }
    max
}

#[inline]
fn lcm(a: u128, b: u128) -> u128 {
    let gcd = gcd(a, b);
    a / gcd * b
}
