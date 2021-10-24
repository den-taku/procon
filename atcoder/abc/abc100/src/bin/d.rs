#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(i64, i64, i64); n]
    }
    let mut dp_l = vec![(0i64, 0i64, 0i64); m + 1];
    let mut dp_s = vec![(0i64, 0i64, 0i64); m + 1];
    for (x, y, z) in xyz {
        //
    }
}
