#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut v = Vec::new();
    for c in s.chars() {
        v.push(c as u64 - b'0' as u64);
    }
    v.reverse();
    let mut ans = 0;
    for i in 0..1 << (v.len() - 1) {
        let mut digit = 1;
        let mut sum = 0;
        for j in 0..v.len() {
            if (i >> j) & 1 == 0 {
                sum += v[j] * digit;
                digit *= 10;
            } else {
                sum += v[j] * digit;
                digit = 1;
            }
        }
        ans += sum;
    }
    println!("{}", ans);
}
