#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String
    }
    if s > t {
        println!("No")
    } else {
        println!("Yes")
    }
}
