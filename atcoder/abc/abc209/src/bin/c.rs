// use bit_sum_library::*;
use proconio::{fastout, input};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut cs: [i64; n]
    }
    cs.sort();
    let first = cs[0] as i64;
    println!(
        "{}",
        cs.iter().skip(1).enumerate().fold(first, |m, (i, e)| {
            m * std::cmp::max(e - (i + 1) as i64, 0) % MOD
        })
    );
    // for i in 1..cs.len() {
    //     // println!("c: {}, count: {}", c, count);
    //     if i >= cs[i] {
    //         println!("0");
    //         return;
    //     } else {
    //         ans *= (cs[i] as i64 - i as i64) % MOD;
    //         ans %= MOD;
    //     }
    // }
    // println!("{}", ans);
}
