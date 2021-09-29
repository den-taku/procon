#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        queries: [(usize, usize); m]
    }
    let mut v = vec![None; n];
    for &(a, b) in &queries {
        if let Some(start) = v[b - 1] {
            if start < a {
                v[b - 1] = Some(a)
            }
        } else {
            v[b - 1] = Some(a)
        }
    }
    let mut count = 0;
    let mut index = 0;
    for (i, &e) in v.iter().enumerate() {
        if let Some(start) = e {
            if start >= index {
                count += 1;
                index = i + 1;
            }
        }
    }
    println!("{}", count);
}
