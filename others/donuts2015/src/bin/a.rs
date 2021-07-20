use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        r: f64,
        d: f64
    }
    println!("{}", PI * r * r * PI * 2.0 * d);
}
