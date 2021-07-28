use proconio::{fastout, input};
use std::f32::consts::PI;

#[fastout]
fn main() {
    input! {
        l1: f32,
        l2: f32,
        l3: f32,
    }
    println!(
        "{}",
        PI * ((l1 + l2 + l3) * (l1 + l2 + l3)
            - max(
                max(
                    wall(l1 - l2 - l3) * wall(l1 - l2 - l3),
                    wall(l3 - l2 - l1) * wall(l3 - l2 - l1)
                ),
                wall(l2 - l3 - l1) * wall(l2 - l3 - l1)
            ))
    )
}

#[inline]
fn wall(a: f32) -> f32 {
    if a > 0.0 {
        a
    } else {
        0.0
    }
}

#[inline]
fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
