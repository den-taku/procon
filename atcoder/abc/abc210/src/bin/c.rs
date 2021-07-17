use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        cs: [u32; n]
    }
    let mut ans;
    let mut now = HashSet::<u32>::new();
    let mut f = HashMap::<u32, usize>::new();
    for &c in cs.iter().take(k) {
        if !now.insert(c) {
            if let Some(&v) = f.get(&c) {
                f.insert(c, v + 1);
            } else {
                f.insert(c, 1);
            }
        }
    }
    ans = now.len();
    for i in k..n {
        if ans >= k {
            break;
        }
        let past = cs[i - k];
        if let Some(&v) = f.get(&past) {
            if v == 1 {
                f.remove(&past);
            } else {
                f.insert(past, v - 1);
            }
        } else {
            now.remove(&past);
        }

        if !now.insert(cs[i].clone()) {
            if let Some(&v) = f.get(&cs[i]) {
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
