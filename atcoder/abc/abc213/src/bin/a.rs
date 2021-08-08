#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32
    }
    println!("{}", a ^ b);
}
