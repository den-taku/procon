#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        w: i64,
        h: i64,
        n: i64,
        p: [(i64, i64, i64); n]
    }
    let mut x_min = -1i64;
    let mut x_max = w as i64 + 1;
    let mut y_min = -1i64;
    let mut y_max = h as i64 + 1;

    for (x, y, a) in p {
        if x_min >= x_max {
            println!("0");
            return;
        }
        if y_min >= y_max {
            println!("0");
            return;
        }
        match a {
            1 => {
                x_min = max(x_min, x);
            }
            2 => {
                x_max = min(x_max, x);
            }
            3 => {
                y_min = max(y_min, y);
            }
            4 => {
                y_max = min(y_max, y);
            }
            _ => unreachable!(),
        }
    }
    let h = h as usize;
    let w = w as usize;

    if x_min >= x_max {
        println!("0");
        return;
    }
    if y_min >= y_max {
        println!("0");
        return;
    }
    let x_min = if x_min == -1 { 0 } else { x_min };
    let x_max = if x_max == w as i64 + 1 {
        x_max - 1
    } else {
        x_max
    };
    let y_min = if y_min == -1 { 0 } else { y_min };
    let y_max = if y_max == h as i64 + 1 {
        y_max - 1
    } else {
        y_max
    };
    println!("{}", (x_max - x_min) * (y_max - y_min));
}
