#![allow(unreachable_code)]
use proconio::input;
use std::cmp::min;

// #[fastout]
fn main() {
    input! {
        d: usize,
        g: usize,
        pc: [(usize, usize); d]
    }
    let mut ans = std::usize::MAX;
    let pc = pc.into_iter().map(|e| (e.0, e.1 / 100)).collect::<Vec<_>>();
    let g = g / 100;
    'out: for i in 0..1 << d {
        let mut sum = 0;
        let mut count = 0;
        for j in 0..d {
            if (i >> j) & 1 == 1 {
                sum += pc[j].0 * (j + 1) + pc[j].1;
                count += pc[j].0;
            }
        }
        if sum >= g {
            ans = min(ans, count);
            continue;
        }
        for j in (0..d).rev() {
            if i >> j & 1 == 0 {
                for _ in 0..(pc[j].0 as usize) {
                    sum += j + 1;
                    count += 1;
                    if sum >= g {
                        ans = min(ans, count);
                        continue 'out;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
