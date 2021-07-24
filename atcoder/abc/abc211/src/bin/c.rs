use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut dp = vec![vec![0; 9]; s.len() + 1];
    dp[0][0] = 1;
    let mut i = 1;
    for c in s.chars() {
        match c {
            'c' => {
                update(&mut dp, i, 1);
            }
            'h' => {
                update(&mut dp, i, 2);
            }
            'o' => {
                update(&mut dp, i, 3);
            }
            'k' => {
                update(&mut dp, i, 4);
            }
            'u' => {
                update(&mut dp, i, 5);
            }
            'd' => {
                update(&mut dp, i, 6);
            }
            'a' => {
                update(&mut dp, i, 7);
            }
            'i' => {
                update(&mut dp, i, 8);
            }
            _ => {
                for j in 0..9 {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        i += 1;
    }
    println!("{}", dp[s.len()][8]);
}

#[inline]
fn update(dp: &mut Vec<Vec<u64>>, index: usize, kind: usize) {
    dp[index][kind] = (dp[index - 1][kind - 1] + dp[index - 1][kind]) % MOD;
    for j in 0..9 {
        if j == kind {
            continue;
        }
        dp[index][j] = dp[index - 1][j];
    }
}
