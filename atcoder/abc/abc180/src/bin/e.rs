use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, i64); n]
    }
    let mut distance = vec![0; n * n];
    for (i, &(a, b, c)) in xyz.iter().enumerate() {
        for (j, &(p, q, r)) in xyz.iter().enumerate() {
            distance[i * n + j] = (p - a).abs() + (q - b).abs() + std::cmp::max(0, r - c);
        }
    }
    let mut dp = vec![None; (1 << (n - 1)) * n];
    dp[0] = Some(0);
    let mut ans = std::i64::MAX;
    for i in 0..n - 1 {
        ans = std::cmp::min(
            ans,
            rec((1 << (n - 1)) - 1, i, &mut dp, n, &distance) + distance[(i + 1) * n],
        );
    }
    println!("{}", ans);
}

fn rec(set: usize, last: usize, dp: &mut [Option<i64>], n: usize, distance: &[i64]) -> i64 {
    if set == 0 {
        0
    } else if let Some(d) = dp[set * n + last] {
        d
    } else {
        let mut d = None;
        let past = set ^ (1 << last);
        if past == 0 {
            d = Some(distance[last + 1])
        } else {
            for i in 0..n - 1 {
                if i == last {
                    continue;
                }
                if set >> i & 1 == 1 {
                    let p = rec(past, i, dp, n, distance);
                    if let Some(est) = d {
                        d = Some(std::cmp::min(est, p + distance[(i + 1) * n + last + 1]))
                    } else {
                        d = Some(p + distance[(i + 1) * n + (last + 1)])
                    }
                }
            }
        }
        dp[set * n + last] = d;
        d.unwrap()
    }
}
