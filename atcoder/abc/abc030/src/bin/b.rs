#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: f64,
        m: f64,
    }
    let minite = 6.0 * m;
    let hour = (30.0 * n + 30.0 * m / 60.0) % 360.0;
    let dis = max(minite, hour) - min(minite, hour);
    println!("{}", min(
        dis,
        360.0 - dis
    ))
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}
