#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        x: isize,
        y: isize
    }
    let mut ans = vec![0; n as usize - 1];
    for i in 1..n {
        for j in i + 1..n + 1 {
            let strait = j - i;
            let short = (x - i).abs() + 1 + (j - y).abs();
            ans[std::cmp::min(strait, short) as usize - 1] += 1;
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
