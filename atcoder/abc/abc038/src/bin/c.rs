#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        array: [usize; n]
    }
    let mut sum = 1u64;
    let mut count = 1;
    for (a, b) in array.iter().zip(array.iter().skip(1)) {
        if a < b {
            count += 1;
            sum += count;
        } else {
            count = 1;
            sum += count;
        }
    }
    println!("{}", sum);
}
