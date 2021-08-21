#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    println!(
        "{}",
        if s == "Hello,World!".to_string() {
            "AC"
        } else {
            "WA"
        }
    )
}
