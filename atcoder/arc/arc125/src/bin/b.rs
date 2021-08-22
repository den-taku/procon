#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        if i * i > n {
            break;
        } else {
            ans += ((n / i - i) >> 1) + 1;
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
