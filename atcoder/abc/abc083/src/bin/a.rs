#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32
    }
    println!(
        "{}",
        if a + b > c + d {
            "Left"
        } else if a + b == c + d {
            "Balanced"
        } else {
            "Right"
        }
    )
}
