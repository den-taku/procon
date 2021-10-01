use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
        c: usize,
    }
    let zero = b'0';
    let digit = n
        .chars()
        .map(|c| (c as u8 - zero) as usize)
        .collect::<Vec<_>>();
    let mut dp = vec![0usize; (c + 1) * 2];
    dp[0] = 1;
    for d in digit {
        let mut next = vec![0usize; (c + 1) * 2];
        for j in 0..2 {
            for k in 0..c + 1 {
                if j == 0 {
                    if d == 0 {
                        next[k] += dp[k];
                    } else {
                        next[c + 1 + k] += dp[k];
                    }
                    if k < c && d > 0 {
                        next[k + 1] += dp[k];
                        next[c + 1 + k + 1] += dp[k] * (d - 1);
                    }
                } else {
                    next[c + 1 + k] += dp[c + 1 + k];
                    if k < c {
                        next[c + 1 + k + 1] += dp[c + 1 + k] * 9;
                    }
                }
            }
        }
        dp = next;
    }
    println!("{}", dp[c] + dp[2 * c + 1])
}

// my first implementation

// use proconio::{fastout, input};

// #[fastout]
// fn main() {
//     input! {
//         n: String,
//         c: usize,
//     }
//     let zero = b'0';
//     let digit = n
//         .chars()
//         .map(|c| (c as u8 - zero) as usize)
//         .collect::<Vec<_>>();
//     println!(
//         "{}",
//         (1..10).map(|e| rec(vec![e], c, &digit)).sum::<usize>()
//     )
// }

// fn rec(v: Vec<usize>, c: usize, digit: &[usize]) -> usize {
//     if v.len() < c {
//         let mut sum = 0;
//         for i in 1..v[v.len() - 1] + 1 {
//             let mut tmp = v.clone();
//             tmp.push(i);
//             sum += rec(tmp, c, digit);
//         }
//         sum
//     } else {
//         // dp[i][j][k]
//         // i: digit
//         // j: true or false
//         // k: use
//         let mut dp = vec![vec![vec![0usize; 1 << c]; 2]; digit.len() + 1];
//         dp[0][0][0] = 1;
//         for i in 0..digit.len() {
//             for j in 0..2 {
//                 for k in 0..1 << c {
//                     if j == 0 {
//                         // the same
//                         // add zero
//                         if digit[i] == 0 {
//                             dp[i + 1][0][k] += dp[i][j][k];
//                         } else {
//                             dp[i + 1][1][k] += dp[i][j][k];
//                         }
//                         for l in 0..c {
//                             if (k >> l) & 1 == 0 {
//                                 if digit[i] == v[l] {
//                                     dp[i + 1][0][k | (1 << l)] += dp[i][j][k];
//                                 } else if digit[i] > v[l] {
//                                     dp[i + 1][1][k | (1 << l)] += dp[i][j][k];
//                                 }
//                             }
//                         }
//                     } else {
//                         dp[i + 1][1][k] += dp[i][j][k];
//                         for l in 0..c {
//                             if (k >> l) & 1 == 0 {
//                                 dp[i + 1][1][k | (1 << l)] += dp[i][j][k];
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         let mut num = 1;
//         if c > 2 && v[0] == v[1] && v[1] == v[2] {
//             num = 6;
//         } else if (c > 2 && (v[0] == v[1] || v[1] == v[2])) || (c > 1 && v[0] == v[1]) {
//             num = 2;
//         }
//         // println!("{:?}: {}, {}", v, dp[digit.len()][0][(1 << c) - 1], dp[digit.len()][1][(1 << c) - 1]);
//         (dp[digit.len()][0][(1 << c) - 1] + dp[digit.len()][1][(1 << c) - 1]) / num
//     }
// }
