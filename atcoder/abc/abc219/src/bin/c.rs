#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String,
        n: usize,
        mut s: [String; n]
    }
    s.sort_by(|a, b| cmp(a, b, &x));
    println!("{}", s.join("\n"));
}

fn cmp(a: &str, b: &str, x: &str) -> std::cmp::Ordering {
    for (a_c, b_c) in a.chars().zip(b.chars()) {
        if a_c == b_c {
            continue;
        }
        let a_i = x.chars().enumerate().find(|(_, c)| c == &a_c).unwrap().0;
        let b_i = x.chars().enumerate().find(|(_, c)| c == &b_c).unwrap().0;
        if a_i < b_i {
            return std::cmp::Ordering::Less;
        } else if a_i > b_i {
            return std::cmp::Ordering::Greater;
        } else {
            continue;
        }
    }
    if a.len() < b.len() {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}
