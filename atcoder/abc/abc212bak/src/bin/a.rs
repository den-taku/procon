#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32
    }
    if 0 < a && b == 0 {
        println!("Gold")
    } else if a == 0 && b > 0 {
        println!("Silver")
    } else {
        println!("Alloy")
    }
}
