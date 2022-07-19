#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [String; h]
    }
    let mut able = vec![true; h * w];
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '.' {
                if i > 0 && j > 0 {
                    able[(i - 1) * w + (j - 1)] = false;
                }
                if i > 0 {
                    able[(i - 1) * w + j] = false;
                }
                if i > 0 && j < w - 1 {
                    able[(i - 1) * w + j + 1] = false
                }
                if j > 0 {
                    able[i * w + j - 1] = false;
                }
                able[i * w + j] = false;
                if j < w - 1 {
                    able[i * w + j + 1] = false;
                }
                if i < h - 1 && j > 0 {
                    able[(i + 1) * w + j - 1] = false;
                }
                if i < h - 1 {
                    able[(i + 1) * w + j] = false;
                }
                if i < h - 1 && j < w - 1 {
                    able[(i + 1) * w + j + 1] = false;
                }
            }
        }
    }

    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                if i > 0 && j > 0 && able[(i - 1) * w + (j - 1)] {
                    continue;
                } else if i > 0 && able[(i - 1) * w + j] {
                    continue;
                } else if i > 0 && j < w - 1 && able[(i - 1) * w + j + 1] {
                    continue;
                } else if j > 0 && able[i * w + j - 1] {
                    continue;
                } else if able[i * w + j] {
                    continue;
                } else if j < w - 1 && able[i * w + j + 1] {
                    continue;
                } else if i < h - 1 && j > 0 && able[(i + 1) * w + j - 1] {
                    continue;
                } else if i < h - 1 && able[(i + 1) * w + j] {
                    continue;
                } else if i < h - 1 && j < w - 1 && able[(i + 1) * w + j + 1] {
                    continue;
                }
                println!("impossible");
                return;
            }
        }
    }

    println!("possible");
    for i in 0..h {
        for j in 0..w {
            if able[i * w + j] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}
