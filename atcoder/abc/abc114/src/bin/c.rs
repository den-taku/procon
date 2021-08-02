#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let v = [0, 3, 5, 7];
    let mut ans = 0;
    for mut i in 27.. {
        let mut value = 0;
        let mut digit = 1;
        while i > 0 {
            value += digit * v[i % 4];
            i /= 4;
            digit *= 10;
        }
        if n >= value && check(value) {
            ans += 1;
        } else if n < value {
            break;
        }
    }
    println!("{}", ans);
}

#[inline]
fn check(mut v: usize) -> bool {
    let mut u = Vec::new();
    while v > 0 {
        u.push(v % 10);
        v /= 10;
    }
    u.sort();
    u.dedup();
    u.len() == 3 && u[0] != 0
}
