use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        s: [i64; n]
    }
    if k == 0 {
        if s.into_iter().any(|x| x == 0) {
            println!("{}", n);
        } else {
            println!("0");
        }
        return;
    }
    let mut best = VecDeque::<i64>::new();
    let mut now = VecDeque::<i64>::new();
    let mut mul = 1;
    for e in s {
        if e == 0 {
            println!("{}", n);
            return;
        }
        now.push_back(e);
        mul *= e;
        if mul > k {
            if best.len() < now.len() {
                let mut tmp = now.clone();
                tmp.pop_back().unwrap();
                best = tmp;
            }
            while mul > k {
                if let Some(el) = now.pop_front() {
                    mul /= el;
                }
            }
        }
    }
    let ans = if best.len() >= now.len() {
        best.len()
    } else {
        now.len()
    };
    println!("{}", ans);
}
