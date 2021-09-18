#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        ss: [String; 3],
        t: String
    }
    let mut s = String::new();
    for c in t.chars() {
        if c == '1' {
            s = format!("{}{}", s, ss[0])
        } else if c == '2' {
            s = format!("{}{}", s, ss[1])
        } else {
            s = format!("{}{}", s, ss[2])
        }
    }
    println!("{}", s);
}
