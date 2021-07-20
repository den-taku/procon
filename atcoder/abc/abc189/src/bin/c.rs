use proconio::{fastout, input};
use std::cmp::max;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    let set: HashSet<i32> = a.iter().cloned().collect();
    let mut ans = 0;
    for &e in set.iter() {
        ans = max(ans, calc(e, &a))
    }
    println!("{}", ans);
}

fn calc(e: i32, a: &[i32]) -> i32 {
    let mut better = 0i32;
    let mut est = 0;
    for &el in a {
        if el >= e {
            est += 1;
        } else {
            better = max(est, better);
            est = 0;
        }
    }
    max(est, better) * e
}
