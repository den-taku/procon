// TDPC-Q (https://atcoder.jp/contests/tdpc/tasks/tdpc_concatenation)
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

const MOD: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        w: [String; n]
    }
    let w: Vec<usize> = w
        .iter()
        .map(|s| {
            let mut i = 0;
            s.chars().rev().fold(0, |e, c| match c {
                '0' => {
                    i += 1;
                    e
                }
                '1' => {
                    i += 1;
                    e + (1 << (i - 1))
                }
                _ => unreachable!(),
            })
        })
        .collect();
    let set: HashSet<usize> = w.iter().cloned().collect();
    // for e in set {
    //     println!("{}", e);
    // }
    // let mut dp = HashMap::<(usize, usize, usize), bool>::with_capacity((l + 1) * (2 << 16));
    // dp.insert((0, 0, 0), true);
    // println!(
    //     "{}",
    //     (0..0b1111111).fold(0, |c, i| {
    //         let mut flag = false;
    //         for j in 0..0b1111111 {
    //             if rec(&set, &mut dp, l, i, 1 + (j << 2)) {
    //                 flag = true;
    //                 break;
    //             }
    //         }
    //         if flag {
    //             (c + 1) % MOD
    //         } else {
    //             c
    //         }
    //     })
    // );
}

// fn rec(
//     set: &HashSet<usize>,
//     dp: &mut HashMap<(usize, usize, usize), bool>,
//     n: usize,
//     i: usize,
//     j: usize,
// ) -> bool {
//     if let Some(f) = dp.get(&(n, i, j)) {
//         *f
//     } else {
//         let mut flag = false;
//         let mut mask = 0;
//         let min = std::cmp::min(n, 8);
//         for k in 1..min {
//             mask += (1 << k);
//             if j << k & 1 == 1 {
//                 if rec(set, dp, n - k, )
//             }
//         }
//         dp.insert((n, i, j), flag);
//         flag
//     }
// }
