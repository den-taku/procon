#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    // let mut v = Vec::new();
    let mut set = std::collections::HashSet::new();
    for _ in 0..n {
        input! { l: usize, a: [usize; l] }
        // v.push(a)
        set.insert(a);
    }
    // v.sort();
    // v.dedup();
    // println!("{}", v.len())
    println!("{}", set.len())
}
