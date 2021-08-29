#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        f: f32
    }
    let x = f as i32;
    let y = (f * 10.0 % 10.0) as i32;
    if 0 <= y && y <= 2 {
        println!("{}-", x);
    } else if 3 <= y && y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
