#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n]
    }
    let mut dp = vec![None; (x + 1) * (y + 1)];
    dp[0] = Some(0);
    for (a, b) in ab.iter().map(|ab| (ab.0, ab.1)) {
        for i in (0..x + 1).rev() {
            for j in (0..y + 1).rev() {
                if let Some(c1) = dp[i * (y + 1) + j] {
                    if let Some(c2) = dp[min(x, i + a) * (y + 1) + min(y, j + b)] {
                        dp[min(x, i + a) * (y + 1) + min(y, j + b)] = Some(min(c1 + 1, c2));
                    } else {
                        dp[min(x, i + a) * (y + 1) + min(y, j + b)] = Some(c1 + 1)
                    }
                }
            }
        }
    }
    if let Some(c) = dp[x * (y + 1) + y] {
        println!("{}", c)
    } else {
        println!("-1")
    }
}

// #![allow(unreachable_code)]
// use proconio::{fastout, input};

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         x: usize,
//         y: usize,
//         ab: [(usize, usize); n]
//     }
//     let asm = ab.iter().map(|e| e.0).sum::<usize>();
//     let bsm = ab.iter().map(|e| e.1).sum::<usize>();
//     if asm < x || bsm < y {
//         println!("-1");
//         return;
//     } else if asm == x {
//         println!("{}", n);
//         return;
//     } else if bsm == y {
//         println!("{}", n);
//         return;
//     } else {
//         let maxx = ab.iter().map(|e| e.0).max().unwrap();
//         let maxy = ab.iter().map(|e| e.1).max().unwrap();
//         let mut dp = vec![vec![None; y + maxy + 3]; x + maxx + 3];
//         for i in 0..n {
//             dp[0][0] = Some(0);
//             let mut next = dp.clone();
//             let a = ab[i].0;
//             let b = ab[i].1;
//             for j in 0..x + maxx + 3 {
//                 for k in 0..y + maxy + 3 {
//                     if a <= j && b <= k {
//                         if let Some(c1) = dp[j - a][k - b] {
//                             if let Some(c2) = dp[j][k] {
//                                 next[j][k] = Some(std::cmp::min(c1 + 1, c2))
//                             } else {
//                                 next[j][k] = Some(c1 + 1)
//                             }
//                         }
//                     }
//                 }
//             }
//             dp = next;
//         }
//         let mut min = std::usize::MAX;
//         for j in x..x + maxx + 3 {
//             for k in y..y + maxy + 3 {
//                 if let Some(c) = dp[j][k] {
//                     min = std::cmp::min(min, c);
//                 }
//             }
//         }
//         if min == std::usize::MAX {
//             println!("-1")
//         } else {
//             println!("{}", min)
//         }
//     }
// }
