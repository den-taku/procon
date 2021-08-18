#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let mut n = 1000 - n;
    let mut count = 0;
    let coins = [500, 100, 50, 10, 5, 1];
    for coin in coins.iter() {
        count += n / coin;
        n %= coin;
    }
    println!("{}", count);
}
