use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n]
    }
    let mut ans = 0;
    let mut count = vec![0i64; n+1];
    for i in 0..n {
        count[b[c[i] - 1]] += 1;
    }
    for i in 0..n {
        ans += count[a[i]];
    }
    println!("{}", ans);
}
