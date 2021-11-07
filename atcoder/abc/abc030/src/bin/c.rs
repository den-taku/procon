#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        a: [usize; n],
        b: [usize; m]
    }
    let mut time = 0;
    let mut count = 0;

    loop {
        let index = a.binary_search(&time).unwrap_or_else(|x| x);
        if index == a.len() {
            break;
        }
        time = a[index] + x;
        let index = b.binary_search(&time).unwrap_or_else(|x| x);
        if index == b.len() {
            break;
        }
        count += 1;
        time = b[index] + y;
    }

    println!("{}", count);
}