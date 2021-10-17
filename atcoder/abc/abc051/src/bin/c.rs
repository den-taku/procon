#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::iter::repeat;

#[fastout]
fn main() {
    input! {
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32
    }
    let mut ans = String::new();
    // for c in vec!['U'].into_iter().cycle().take((ty - sy) as usize) {
    //     ans.push(c);
    // }
    for c in repeat('U').take((ty - sy) as usize) {
        ans.push(c);
    }
    for c in repeat('R').take((tx - sx) as usize) {
        ans.push(c);
    }
    for c in repeat('D').take((ty - sy) as usize) {
        ans.push(c);
    }
    for c in repeat('L').take((tx - sx) as usize) {
        ans.push(c);
    }
    ans.push('L');
    for c in repeat('U').take((ty - sy) as usize + 1) {
        ans.push(c);
    }
    for c in repeat('R').take((tx - sx) as usize + 1) {
        ans.push(c);
    }
    ans.push('D');
    ans.push('R');
    for c in repeat('D').take((ty - sy) as usize + 1) {
        ans.push(c);
    }
    for c in repeat('L').take((tx - sx) as usize + 1) {
        ans.push(c);
    }
    ans.push('U');

    println!("{}", ans);
}
