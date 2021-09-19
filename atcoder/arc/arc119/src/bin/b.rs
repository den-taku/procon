#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
        t: String
    }
    if s.chars().filter(|&c| c == '1').count() != t.chars().filter(|&c| c == '1').count() {
        println!("-1");
        return;
    }
    let mut s = s
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let mut t = t
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        if s[i] == t[i] {
            continue;
        }
        if s[i] == 0 {
            let index = t.iter().enumerate().skip(i).find(|e| e.1 == &0).unwrap().0;
            t.swap(i, index);
        } else {
            let index = s.iter().enumerate().skip(i).find(|e| e.1 == &0).unwrap().0;
            s.swap(i, index);
        }
        ans += 1;
    }
    println!("{}", ans);
}
