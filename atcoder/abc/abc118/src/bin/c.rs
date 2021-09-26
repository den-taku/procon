#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut a = a.into_iter().collect::<BTreeSet<_>>();
    a.insert(0);
    while a.len() > 2 {
        let &one = a.iter().last().unwrap();
        a.remove(&one);
        let &two = a.iter().last().unwrap();
        a.insert(one % two);
    }
    for e in a.iter().last() {
        println!("{}", e)
    }
}
