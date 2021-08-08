#![allow(unreachable_code)]
use proconio::{fastout, input};

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
    println!("");
}

#[inline]
fn rec(index: usize, ajacent: &[Vec<usize>], used: &mut Vec<bool>, v: &mut String) {
    print!("{} ", index + 1);
    used[index] = true;
    for &e in &ajacent[index] {
        if !used[e] {
            rec(e, ajacent, used, v);
            print!("{} ", index + 1);
        }
    }
}
