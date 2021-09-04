#![allow(unreachable_code)]
use proconio::{fastout, input};

const MOD: u64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        matches: [(usize, usize); m]
    }
    let mut dp = vec![vec![0; 2 * n + 1]; 2 * n + 1];
    for i in 0..2 * n + 1 {
        dp[i][0] = 1;
    }
    let mut friend = vec![vec![0; 2 * n + 1]; 2 * n + 1];
    for (x, y) in matches {
        friend[x][y] = 1;
        friend[y][x] = 1;
    }
    for j in 1..n + 1 {
        for i in 0..2 * (n - j) + 1 {
            for k in 0..j {
                dp[i][j] += friend[i][i + 2 * k + 1] * dp[i + 1][k] * dp[i + 2 * k + 2][j - k - 1]
                    % MOD
                    * jCk(j, k + 1)
                    % MOD;
                dp[i][j] %= MOD;
            }
        }
    }
    println!("{}", dp[0][n]);
}

fn jCk(j: usize, k: usize) -> u64 {
    fact(j) / fact(k) / fact(j - k)
}

fn fact(i: usize) -> u64 {
    if i == 1 || i == 0 {
        1
    } else {
        i as u64 * fact(i - 1)
    }
}
