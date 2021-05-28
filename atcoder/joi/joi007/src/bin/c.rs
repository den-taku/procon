// use proconio::{fastout, input};
// use std::cmp::Reverse;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         m: i64,
//         mut p: [i64; n]
//     }
//     p.sort_by_key(|&x| Reverse(x));
//     let mut est = 0;
//     for i in 0..n {
//         if p[i] >= m || p[i] * 4 <= est {
//             break;
//         }
//         let cnd = first(i, n, m, &p);
//         if cnd > est {
//             est = cnd;
//         }
//     }
//     println!("{}", est);
// }

// fn first(i: usize, n: usize, m: i64, p: &Vec<i64>) -> i64 {
//     let cnds = [p[i]; 4];
//     let mut est = cnds[0];
//     for j in i..n {
//         if cnds[0] + p[j] * 3 <= est { 
//             break;
//         }
//         if cnds[0] + p[j] >= m {
//             continue;
//         }
//         let cnd = second(j, n, m, p, cnds);
//         if cnd > est {
//             est = cnd;
//         }
//     }
//     return est;
// }

// fn second(j: usize, n: usize, m: i64, p: &Vec<i64>, mut cnds: [i64; 4]) -> i64 {
//     cnds[1] = p[j];
//     let mut est = cnds[0] + cnds[1];
//     let base = cnds[0] + cnds[1];
//     for k in j..n {
//         if base + p[k] * 2 <= est {
//             break;
//         }
//         if base + p[k] >= m {
//             continue;
//         }
//         let cnd = third(k, n, m, p, cnds);
//         if cnd > est {
//             est = cnd;
//         }
//     }
//     return est;
// }

// fn third(k: usize, n: usize, m: i64, p: &Vec<i64>, mut cnds: [i64; 4]) -> i64 {
//     cnds[2] = p[k];
//     let base = cnds[0] + cnds[1] + cnds[2];
//     for l in k..n {
//         if base + p[l] < m {
//             return base + p[l];
//         }
//     }
//     return base;
// }
