#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: usize,
        t: usize
    }
    let mut count = 0;
    for i in 0..s + 1 {
        for j in 0..s + 1 {
            for k in 0..s + 1 {
                if i + j + k <= s && i * j * k <= t {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}
