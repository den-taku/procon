#![allow(unreachable_code)]

fn main() {
    let inputs = input();
    println!(
        "{}",
        inputs
            .iter()
            .map(|(a, b)| culc_dp(a, b))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[inline]
fn culc_dp(a: &str, b: &str) -> String {
    let mut dp = vec![0; a.len() + 1];
    for c in b.chars() {
        let mut next = dp.clone();
        for (i, d) in a.chars().enumerate() {
            let past = std::cmp::max(dp[i + 1], next[i]);
            if c == d {
                next[i + 1] = std::cmp::max(past, dp[i] + 1);
            } else {
                next[i + 1] = past;
            }
        }
        dp = next;
    }
    dp[a.len()].to_string()
}

#[inline]
fn input() -> Vec<(String, String)> {
    let n = read_line::<usize>()[0];
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let mut a = String::new();
        let mut b = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        std::io::stdin().read_line(&mut b).unwrap();
        v.push((
            a.trim().split_whitespace().collect::<Vec<_>>()[0].to_string(),
            b.trim().split_whitespace().collect::<Vec<_>>()[0].to_string(),
        ))
    }
    v
}

#[inline]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}
