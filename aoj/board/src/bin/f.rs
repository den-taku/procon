// Coin Changing Problem(https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A&lang=jp)
#![allow(unreachable_code)]

fn main() {
    let (n, coins) = input();
    let mut memo = vec![0; n + 1];
    for i in 1..n + 1 {
        memo[i] = coins
            .iter()
            .map(|&c| {
                if c <= i {
                    memo[i - c] + 1
                } else {
                    std::usize::MAX
                }
            })
            .min()
            .unwrap_or(std::usize::MAX);
    }
    println!("{}", memo[n]);
}

#[inline]
fn input() -> (usize, Vec<usize>) {
    let n = read_line::<usize>()[0];
    (n, read_line::<usize>())
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
