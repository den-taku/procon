use proconio::{fastout, input};
use std::collections::HashMap;

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64
    }
    let mut dp = HashMap::<(u64, u64), u64>::new();
    println!("{}", { rec(n, m, k, &mut dp) })
}

fn rec(n: u64, m: u64, k: u64, dp: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(e) = dp.get(&(n, m)) {
        *e
    } else if n == 0 {
        0
    } else if n > 0 && m == 0 {
        n.pow(k as u32) % MOD
    } else {
        (rec(n - 1, m, k, dp) % MOD + rec(n, m - 1, k, dp) % MOD) % MOD
    }
}
