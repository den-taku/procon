#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
        x: String,
    }
    let s = s
        .chars()
        .map(|c| c as u8 - b'0')
        .map(|c| c as usize)
        .collect::<Vec<_>>();
    let x = x.chars().collect::<Vec<_>>();
    let mut dp = HashSet::new();
    dp.insert(0);
    for i in (0..n).rev() {
        let mut next = HashSet::new();
        if x[i] == 'T' {
            for num in 0..7 {
                if dp.contains(&((10 * num) % 7)) || dp.contains(&((10 * num + s[i]) % 7)) {
                    next.insert(num);
                }
            }
        } else {
            for num in 0..7 {
                if dp.contains(&((10 * num) % 7)) && dp.contains(&((10 * num + s[i]) % 7)) {
                    next.insert(num);
                }
            }
        }
        dp = next;
    }
    if dp.contains(&0) {
        println!("Takahashi")
    } else {
        println!("Aoki")
    }
}
