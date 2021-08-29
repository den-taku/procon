#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut ans = Vec::new();
    while n > 0 {
        if n % 2 == 0 {
            ans.push("B");
            n /= 2;
        } else {
            ans.push("A");
            n -= 1;
        }
    }
    ans.reverse();
    println!("{}", ans.join(""));
}
