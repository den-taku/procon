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
    let mut ans = vec![std::usize::MAX; n];
    for set in 0..1 << n - 1 {
        let mut closed = vec![false; n];
        for i in 0..n {
            if set >> i & 1 == 1 {
                closed[i] = true;
            }
        }
        let selled = closed.iter().filter(|e| **e).count();
        let money = x.iter().take(selled).sum::<usize>();
        let mut point = 0usize;
        for i in 0..n {
            if closed[i] {
                continue;
            }
            if c[i] <= money {
                point = std::cmp::max(point, v[i])
            }
        }
        ans[selled] = std::cmp::min(ans[selled], point)
    }
    println!("{}", ans.iter().max().unwrap());
}
