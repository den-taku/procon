#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u32,
        a: [u32; n]
    }
    let mut divisors = HashSet::new();
    for e in a {
        divisor(e, &mut divisors);
    }
    divisors.remove(&1);
    let mut ans = Vec::new();
    for i in 1..m + 1 {
        let mut d = HashSet::new();
        divisor(i, &mut d);
        if d.iter().all(|e| !divisors.contains(&e)) {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    for e in ans {
        println!("{}", e);
    }
}

#[inline]
fn divisor(a: u32, set: &mut HashSet<u32>) {
    let mut i = 1;
    while i * i <= a {
        if a % i == 0 {
            set.insert(i);
            if i != a / i {
                set.insert(a / i);
            }
        }
        i += 1;
    }
}
