#![allow(unreachable_code)]
use proconio::{fastout, input};

const ONE: u8 = b'0';

#[fastout]
fn main() {
    input! {
        k: u64,
        a: String,
        b: String
    }
    let mut a_10 = 0;
    let mut b_10 = 0;
    let mut d = 1;
    for e in a.chars().rev() {
        a_10 += (e as u8 - ONE) as u64 * d;
        d *= k
    }
    let mut d = 1;
    for e in b.chars().rev() {
        b_10 += (e as u8 - ONE) as u64 * d;
        d *= k;
    }
    println!("{}", a_10 * b_10);
}
