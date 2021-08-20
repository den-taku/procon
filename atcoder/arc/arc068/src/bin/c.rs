#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64
    }
    let div = x / 11;
    let mo = if x % 11 > 6 {
        2
    } else if x % 11 == 0 {
        0
    } else {
        1
    };
    println!("{}", mo + div * 2);
}
