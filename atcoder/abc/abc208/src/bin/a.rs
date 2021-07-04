use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }
    println!("{}", if b >= a && b <= 6 * a { "Yes" } else { "No" })
}

fn rec(n: u64, m: u64, k: u64) -> u64 {
    if n == 0 {
        0
    } else if n > 0 && m == 0 {
        n.pow(k as u32) % MOD
    } else {
        (rec(n - 1, m, k) % MOD + rec(n, m - 1, k) % MOD) % MOD
    }
}
