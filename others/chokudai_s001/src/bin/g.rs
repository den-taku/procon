#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut digit = 1;
    let mut ans = 0;
    for e in a.into_iter().rev() {
        let haba = {
            let mut n = e;
            let mut i = 0;
            while n > 0 {
                n /= 10;
                i += 1;
            }
            i
        };
        let e = e % MOD;
        ans += e * digit;
        ans %= MOD;
        digit *= 10u64.pow(haba as u32);
        digit %= MOD;
    }
    println!("{}", ans);
}
