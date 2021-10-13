#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String; n]
    }
    let mut count = [std::usize::MAX; 26];
    for s in ss {
        let mut new = [0; 26];
        for c in s.chars() {
            new[(c as u8 - b'a') as usize] += 1;
        }
        for (c, n) in count.iter_mut().zip(new.iter()) {
            *c = std::cmp::min(*c, *n);
        }
    }
    let mut ans = String::new();
    for (c, &i) in count.iter().enumerate() {
        let c = (c as u8 + b'a') as char;
        for _ in 0..i {
            ans.push(c)
        }
    }
    if !ans.is_empty() {
        ans.push('\n');
    }
    print!("{}", ans);
}
