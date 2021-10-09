#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [String; 2 * n]
    }
    let a = a
        .into_iter()
        .map(|v| {
            v.chars()
                .map(|c| {
                    if c == 'G' {
                        0
                    } else if c == 'C' {
                        1
                    } else {
                        2
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let mut v = (1..2 * n + 1).map(|i| (Reverse(0), i)).collect::<Vec<_>>();
    for j in 0..m {
        for i in 0..n {
            let one = v[2 * i];
            let two = v[2 * i + 1];
            let h1 = a[(one.1 - 1) * m + j];
            let h2 = a[(two.1 - 1) * m + j];
            let Reverse(one0) = one.0;
            let Reverse(two0) = two.0;
            if h1 == 0 {
                if h2 == 0 {
                } else if h2 == 1 {
                    v[2 * i] = (Reverse(one0 + 1), one.1);
                    v[2 * i + 1] = (two.0, two.1);
                } else {
                    v[2 * i] = (one.0, one.1);
                    v[2 * i + 1] = (Reverse(two0 + 1), two.1);
                }
            } else if h1 == 1 {
                if h2 == 0 {
                    v[2 * i] = (one.0, one.1);
                    v[2 * i + 1] = (Reverse(two0 + 1), two.1);
                } else if h2 == 1 {
                } else {
                    v[2 * i] = (Reverse(one0 + 1), one.1);
                    v[2 * i + 1] = (two.0, two.1);
                }
            } else {
                if h2 == 0 {
                    v[2 * i] = (Reverse(one0 + 1), one.1);
                    v[2 * i + 1] = (two.0, two.1);
                } else if h2 == 1 {
                    v[2 * i] = (one.0, one.1);
                    v[2 * i + 1] = (Reverse(two0 + 1), two.1);
                } else {
                }
            }
        }
        v.sort();
    }
    for (_n, r) in v {
        println!("{}", r);
    }
}
