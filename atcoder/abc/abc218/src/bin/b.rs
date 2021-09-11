#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: [u8; 26]
    }
    let s = p
        .iter()
        .map(|&e| (e - 1 + 'a' as u8) as char)
        .collect::<String>();
    println!("{}", s)
}
