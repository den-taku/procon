#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n]
    }
    println!("{}", a.iter().filter(|&e| e < &p).count())
}
