#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String
    }
    let mut s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    if s == t {
        println!("Yes");
    } else {
        for i in 0..s.len() - 1 {
            s.swap(i, i + 1);
            if s == t {
                println!("Yes");
                return;
            }
            s.swap(i, i + 1);
        }
        println!("No")
    }
}
