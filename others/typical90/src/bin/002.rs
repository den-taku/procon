#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    if n % 2 != 0 {
    } else {
        for i in 0..1 << n {
            if valid(n, i) {
                println!("{}", decode(n, i));
            }
        }
    }
}

#[inline]
fn valid(n: usize, i: usize) -> bool {
    let mut stack = Vec::new();
    for j in 0..n {
        if i >> j & 1 == 1 {
            stack.push(())
        } else if stack.is_empty() {
            return false;
        } else {
            stack.pop();
        }
    }
    stack.is_empty()
}

#[inline]
fn decode(n: usize, i: usize) -> String {
    let mut chars = Vec::new();
    for j in (0..n).rev() {
        chars.push(if i >> j & 1 == 1 { ')' } else { '(' })
    }
    chars.iter().collect()
}
