#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i128; n]
    }
    let mut sum = vec![0; n + 1];
    for i in 1..n + 1 {
        if i % 2 != 0 {
            sum[i] += a[i - 1]
        } else {
            sum[i] -= a[i - 1]
        }
        sum[i] += sum[i - 1];
    }
    let mut map = std::collections::HashMap::new();
    for s in sum {
        *map.entry(s).or_insert(0) += 1;
    }
    println!(
        "{}",
        map.iter()
            .map(|e| e.1)
            .map(|&e| if e >= 2 { e * (e - 1) / 2 } else { 0 })
            .sum::<i128>()
    )
}
