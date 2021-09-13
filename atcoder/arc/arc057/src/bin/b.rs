#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    if a.iter().sum::<usize>() == k {
        println!("1");
        return;
    }
    let mut dp = vec![None; (n + 1) * n];
    let mut sum = vec![0; n];
    sum[0] = a[0];
    for i in 1..n {
        sum[i] += sum[i - 1];
        sum[i] += a[i];
    }
    for (_j, p) in dp.iter_mut().skip(1).enumerate().take(n) {
        *p = Some((1, 1));
    }
    for i in 1..n {
        for j in 1..n + 1 {
            if let Some((p1, _c1)) = dp[i * (n + 1) + j - 1] {
                if let Some((p2, _c2)) = dp[(i - 1) * (n + 1) + j - 1] {
                    let new_k = (p2 * a[j - 1]) / sum[j - 2] + 1;
                    if new_k + p2 > k || p1 < new_k + p2 || new_k > a[j - 1] {
                        dp[i * (n + 1) + j] = Some((p1, 0));
                    } else {
                        dp[i * (n + 1) + j] = Some((p2 + new_k, new_k));
                    }
                } else {
                    dp[i * (n + 1) + j] = Some((p1, 0));
                }
            } else if let Some((p2, _c2)) = dp[(i - 1) * (n + 1) + j - 1] {
                let new_k = (p2 * a[j - 1]) / sum[j - 2] + 1;
                if new_k + p2 <= k && new_k <= a[j - 1] {
                    dp[i * (n + 1) + j] = Some((p2 + new_k, new_k));
                }
            }
        }
    }
    for i in (0..n).rev() {
        if let Some((mut p, mut c)) = dp[i * (n + 1) + n] {
            if p == k {
                println!("{}", i + 1);
                return;
            } else {
                let mut d = k - p;
                let mut past = 100.1;
                let mut index = i;
                for j in (1..n + 1).rev() {
                    let mut x = a[j - 1] - p;
                    if x > 0 {
                        while {
                            if ((p + x) as f64 * 100.0 / sum[j - 1] as f64) < past {
                                false
                            } else {
                                x -= 1;
                                x > 0
                            }
                        } {}
                        if d < x {
                            d = 0;
                        } else {
                            d -= x;
                        }
                    }
                    past = (p + x) as f64 * 100.0 / sum[j - 1] as f64;
                    if d == 0 {
                        println!("{}", i + 1);
                        return;
                    }
                    if c > 0 {
                        if index == 0 {
                            break;
                        }
                        index -= 1;
                        p = dp[index * (n + 1) + j - 1].unwrap().0;
                        c = dp[index * (n + 1) + j - 1].unwrap().1;
                    } else {
                        p = dp[index * (n + 1) + j - 1].unwrap().0;
                        c = dp[index * (n + 1) + j - 1].unwrap().1;
                    }
                }
            }
        }
    }
    println!("{}", 0);
}
