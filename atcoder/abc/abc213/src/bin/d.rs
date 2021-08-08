#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }
    let mut ajacent = vec![Vec::new(); n];
    for &(a, b) in &ab {
        ajacent[a - 1].push(b - 1);
        ajacent[b - 1].push(a - 1);
    }
    for v in ajacent.iter_mut() {
        v.sort();
    }
    let mut used = vec![false; n];
    let mut v = String::new();
    rec(0, &ajacent, &mut used, &mut v);
    println!("{}", v.split_at(v.len() - 1).0);
}

#[inline]
fn rec(index: usize, ajacent: &[Vec<usize>], used: &mut [bool], v: &mut String) {
    write!(v, "{} ", index + 1).unwrap();
    used[index] = true;
    for &e in &ajacent[index] {
        if !used[e] {
            rec(e, ajacent, used, v);
            write!(v, "{} ", index + 1).unwrap();
        }
    }
}
