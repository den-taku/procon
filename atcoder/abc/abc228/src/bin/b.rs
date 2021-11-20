#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }
    let mut flag = vec![false; n];
    rec(x - 1, &mut flag, &a);
    println!("{}", flag.iter().filter(|e| **e).count())
}

fn rec(index: usize, flag: &mut [bool], a: &[usize]) {
    if !flag[index] {
        flag[index] = true;
        rec(a[index] - 1, flag, a)
    }
}
