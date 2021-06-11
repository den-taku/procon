use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    }
    let mut edge = [1; 16 * 16];
    for &(x, y) in &edges {
        // if a -> b (means a is faster than b), edge[a][b] = 0
        edge[(x - 1) * 16 + (y - 1)] = 0;
    }
    let edge = edge;
    let mut dp = [std::option::Option::<i64>::None; 2 << 16];
    println!("{}", rec(n, (1 << n) - 1, &edge, &mut dp));
}

fn rec(n: usize, set: i64, edge: &[i32], dp: &mut [Option<i64>]) -> i64 {
    if let Some(ans) = dp[set as usize] {
        // already calculated
        ans
    } else if set == 0 {
        // set is empty
        dp[set as usize] = Some(1);
        1
    } else {
        let mut ans = 0;
        (0..n).fold((), |_, i| {
            if set >> i & 1 == 1 && condition(n, edge, set, i) {
                // i can be a last element, then
                ans += rec(n, set - (1 << i), edge, dp);
            }
        });
        dp[set as usize] = Some(ans);
        ans
    }
}

// either cand is able to be a last element or not
fn condition(n: usize, edge: &[i32], set: i64, cand: usize) -> bool {
    (0..n).fold(1, |m, i| {
        if set >> i & 1 == 1 {
            m * edge[cand * 16 + i]
        } else {
            m
        }
    }) != 0
}
