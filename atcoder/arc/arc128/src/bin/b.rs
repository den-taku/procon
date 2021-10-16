#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        t: usize,
        rgb: [(u64, u64, u64); t]
    }
    for (r, g, b) in rgb {
        let mut value = std::u64::MAX;
        if let Some(ans) = calc(r, g) {
            value = ans;
        }
        if let Some(ans) = calc(g, b) {
            value = min(value, ans);
        }
        if let Some(ans) = calc(b, r) {
            value = min(value, ans)
        }
        if value != std::u64::MAX {
            println!("{}", value);
        } else {
            println!("-1");
        }
    }
}

fn calc(mut a: u64, mut b: u64) -> Option<u64> {
    let v = min(a, b);
    a -= v;
    b -= v;
    let a = max(a, b);
    if a % 3 != 0 {
        None
    } else {
        Some(a + v)
    }
}
