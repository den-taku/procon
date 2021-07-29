#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String
    }
    let mut ans = VecDeque::new();
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(true);
            ans.push_back('(');
        } else if stack.is_empty() {
            ans.push_front('(');
            ans.push_back(')');
        } else {
            stack.pop();
            ans.push_back(')');
        }
    }
    for _ in stack {
        ans.push_back(')');
    }
    for e in ans {
        print!("{}", e);
    }
    println!();
}
