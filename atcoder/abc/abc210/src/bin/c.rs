use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        cs: [i64; n]
    }
    let mut ans;
    let mut now = HashSet::<i64>::new();
    let mut f = HashMap::<i64, usize>::new();
    for i in 0..k {
        if !now.insert(cs[i].clone()) {
            if let Some(v) = f.get(&cs[i]) {
                f.insert(cs[i], v + 1);
            } else {
                f.insert(cs[i], 1);
            }
        }
    }
    ans = now.len();
    for i in k..n {
        if ans >= k {
            break;
        }
        let past = cs[i - k].clone();
        let hoge = f.get(&past);
        if let Some(v) = hoge {
            if *v == 1 {
                f.remove(&past);
            } else {
                f.insert(past, v - 1);
            }
        } else {
            now.remove(&past);
        }

        if !now.insert(cs[i].clone()) {
            if let Some(v) = f.get(&cs[i]) {
                f.insert(cs[i], v + 1);
            } else {
                f.insert(cs[i], 1);
            }
        }

        ans = std::cmp::max(ans, now.len());
    }
    ans = std::cmp::max(ans, now.len());
    println!("{}", ans);
}
