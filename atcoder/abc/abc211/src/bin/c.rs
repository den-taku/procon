use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut dp = vec![vec![(0, 0); 8]; s.len() + 1];
    let mut i = 1;
    for c in s.chars() {
        match c {
            'c' => {
                update(&mut dp, i, 0);
            }
            'h' => {
                update(&mut dp, i, 1);
            }
            'o' => {
                update(&mut dp, i, 2);
            }
            'k' => {
                update(&mut dp, i, 3);
            }
            'u' => {
                update(&mut dp, i, 4);
            }
            'd' => {
                update(&mut dp, i, 5);
            }
            'a' => {
                update(&mut dp, i, 6);
            }
            'i' => {
                update(&mut dp, i, 7);
            }
            _ => {
                for j in 0..8 {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        i += 1;
    }
    println!("{}", dp[s.len()][7]);
}

#[inline]
fn update(dp: &mut Vec<Vec<(u64, u64)>>, index: usize, kind: usize) {
    dp[index][kind].0 = (dp[index - 1][kind] + 1) % MOD;
    for j in 0..8 {
        if j == kind {
            continue;
        }
        dp[index][j] = dp[index - 1][j];
    }
    for j in 1..8 {
        dp[index][j].1 = dp[index][j - 1].1 * dp[index][j].0;
    }
}
