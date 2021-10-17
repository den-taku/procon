#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }
    let point = ab.iter().copied().map(|(a, b)| a / b).sum::<f64>() / 2.0;
    let mut sum = 0.0;
    let mut sum2 = 0.0;
    for (a, b) in ab {
        sum += a;
        sum2 += a / b;
        if sum2 >= point {
            sum2 -= a / b;
            let diff = point - sum2;
            sum -= a;
            sum += b * diff;
            println!("{}", sum);
            return;
        }
    }
}
