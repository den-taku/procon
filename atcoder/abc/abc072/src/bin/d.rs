#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [usize; n]
    }
    p.push(p.len() + 2);
    let mut count = 0;
    for i in 0..p.len() {
        if p[i] == i + 1 && p[i + 1] == i + 2 {
            p[i + 1] += 1;
            count += 1;
        } else if p[i] == i + 1 {
            count += 1;
        }
    }
    println!("{}", count);
}
