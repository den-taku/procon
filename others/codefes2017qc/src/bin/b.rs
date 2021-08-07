#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    println!("{}", rec(a));
}

#[inline]
fn rec(mut a: Vec<i64>) -> u64 {
    if let Some(v) = a.pop() {
        if v % 2 == 0 {
            let se = 3u64.pow(a.len() as u32);
            let m1 = rec(a);
            m1 * 2 + se
        } else {
            let m1 = 3u64.pow(a.len() as u32);
            let se = rec(a);
            m1 * 2 + se
        }
    } else {
        0
    }
}
