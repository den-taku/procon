use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [usize; n],
        c: [usize; n],
        v: [usize; n]
    }
    let mut dp = vec![None; (1 << n) * n];
    dp[0] = Some(0);
    let mut ans = std::i64::MAX;
    for i in 0..n {}
    println!("{}", ans);
}
