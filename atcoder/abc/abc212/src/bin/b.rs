#![allow(unreachable_code)]
use proconio::{fastout, input};

const ZERO: u8 = b'0';

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut v = Vec::new();
    for c in s.chars() {
        v.push(c as u8 - ZERO);
    }
    let first = v[0];
    let mut u = v.clone();
    u.sort();
    u.dedup();
    if u.len() == 1 {
        println!("Weak");
        return;
    }
    for (i, a) in v.iter().skip(1).enumerate() {
        if a != &((first + i as u8 + 1) % 10) {
            println!("Strong");
            return;
        }
    }

    println!("Weak");
}
