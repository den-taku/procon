#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }
    let t = [1, 1, 4, 4, 2, 1, 1, 4, 4, 2];
    println!("{}", pow(a % 10, index(b % t[a % 10], c, t[a % 10]), 10))
}

#[inline(always)]
fn pow(a: usize, x: usize, m: usize) -> usize {
    if x == 0 {
        1
    } else if x == 1 {
        a % m
    } else if x % 2 == 0 {
        let a_x_2 = pow(a, x / 2, m);
        (a_x_2 * a_x_2) % m
    } else {
        let a_x_2 = pow(a, x / 2, m);
        (a_x_2 * a_x_2 * a) % m
    }
}

#[inline(always)]
fn index(b: usize, c: usize, t: usize) -> usize {
    let pow_num = pow(b, c, t);
    if pow_num == 0 {
        t
    } else {
        pow_num
    }
}
