#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut v = Vec::new();
    for c in s.chars() {
        v.push(c as i64 - b'0' as i64)
    }
    for i in 0..1 << 3 {
        let mut sum = v[0];
        for j in 0..3 {
            if i >> j & 1 == 0 {
                sum -= v[j + 1]
            } else {
                sum += v[j + 1]
            }
        }
        if sum == 7 {
            println!(
                "{}{}{}{}{}{}{}=7",
                v[0],
                convert(i, 0),
                v[1],
                convert(i, 1),
                v[2],
                convert(i, 2),
                v[3],
            );
            break;
        }
    }
}

#[inline]
fn convert(i: usize, j: usize) -> char {
    if i >> j & 1 == 0 {
        '-'
    } else {
        '+'
    }
}
