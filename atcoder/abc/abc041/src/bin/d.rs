use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    }
    let mut edge = [0; 16 * 16];
    for &(x, y) in &edges {
        edge[(x - 1) * 16 + (y - 1)] = 1;
    }
    let edge = edge;
    let mut dp = [std::option::Option::<i64>::None; (2 << 16) * 16];
    //
    let mut ans: i64 = 0;
    let _: Vec<()> = (0..n)
        .map(|i| {
            ans = rec(n, 0, i, &edge, &mut dp);
        })
        .collect();
    println!("{}", ans);
}

fn rec(n: usize, set: i64, v: usize, edge: &[i32], dp: &mut [Option<i64>]) -> i64 {
    if let Some(ans) = dp[set as usize * 16 + v] {
        ans
    } else if set == (2 << n) - 1 {
        dp[set as usize * 16 + v] = Some(1);
        1
    } else {
        let mut ans = 0;
        let _: Vec<()> = (0..n)
            .map(|i| {
                if (set >> i) & 1 != 1 {
                    if edge[v * 16 + i] != 0 {
                        ans += rec(n, set + (1 << i), i, edge, dp);
                    }
                }
            })
            .collect();
        dp[set as usize * 16 + v] = Some(ans);
        ans
    }
}
