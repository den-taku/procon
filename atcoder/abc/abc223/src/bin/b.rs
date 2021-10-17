#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut v = Vec::new();
    for i in 0..s.len() {
        v.push(s.chars().cycle().skip(i).take(s.len()).collect::<String>());
    }
    v.sort();
    println!("{}", v[0]);
    println!("{}", v[v.len() - 1])
}
