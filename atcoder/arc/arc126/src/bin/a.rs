#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        t: usize,
        n234: [(u64, u64, u64); t]
    }
    let mut ans = Vec::new();
    for ((mut a, mut b, mut c), (mut d, mut e, mut f)) in n234.iter().zip(n234.iter()) {
        let n442 = {
            let s = min(c / 2, a);
            c -= s * 2;
            a -= s;
            s
        };
        let n433 = {
            let s = min(b / 2, c);
            b -= 2 * s;
            c -= s;
            s
        };
        let n4222 = {
            let s = min(a / 3, c);
            a -= 3 * s;
            c -= s;
            s
        };
        let n3322 = {
            let s = min(a / 2, b / 2);
            a -= 2 * s;
            b -= s * 2;
            s
        };
        let n22222 = a / 5;
        let one = n442 + n433 + n4222 + n3322 + n22222;
        let n433 = {
            let s = min(e / 2, f);
            e -= 2 * s;
            f -= s;
            s
        };
        let n442 = {
            let s = min(f / 2, d);
            f -= s * 2;
            d -= s;
            s
        };
        let n4222 = {
            let s = min(d / 3, f);
            d -= 3 * s;
            f -= s;
            s
        };
        let n3322 = {
            let s = min(d / 2, e / 2);
            d -= 2 * s;
            e -= s * 2;
            s
        };
        let n22222 = d / 5;
        let two = n442 + n433 + n4222 + n3322 + n22222;
        ans.push(std::cmp::max(one, two).to_string());
    }
    println!("{}", ans.join("\n"))
}
