#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String
    }
    let mut count = 0;
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(true);
        } else if stack.is_empty() {
            count += 1;
        } else {
            stack.pop();
        }
    }
    for _ in 0..count {
        print!("(");
    }
    print!("{}", s);
    for _ in stack {
        print!(")");
    }
    println!();
}
