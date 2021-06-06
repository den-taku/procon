use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }
    let sum = t.iter().fold(0, |i, e| i + e);
    let sum_2 = (sum+1) / 2;
    let mut dp = vec![false; (sum + 1) * (n + 1)];
    for i in 0..n + 1 {
        dp[i * (sum + 1)] = true;
    }
    for j in 1..sum + 1 {
        for i in 1..n + 1 {
            dp[i * (sum + 1) + j] = if j >= t[i - 1] {
                dp[(i - 1) * (sum + 1) + j] || dp[(i - 1) * (sum + 1) + j - t[i - 1]]
            } else {
                dp[(i - 1) * (sum + 1) + j]
            };
        }
        if j >= sum_2 && dp[n * (sum + 1) + j] {
            println!("{}", j);
            return;
        }
    }
}
