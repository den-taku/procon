use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        ns: [i128; t]
    }
    for n in ns {
        println!("{}", calc(n));
    }
}

#[inline]
fn calc(mut n: i128) -> i32 {
    use std::cmp::max;
    let mut v = Vec::new();
    while n > 0 {
        v.push((n % 10) as i32);
        n /= 10;
    }
    let mut dp = vec![(0i32, 0); v.len()];
    match v[0] {
        0 => dp[0] = (4, 0),
        1 | 2 | 3 => dp[0] = (1, v[0]),
        4 | 5 | 6 => dp[0] = (2, v[0]),
        7 | 8 | 9 => dp[0] = (3, v[0]),
        _ => unreachable!(),
    }
    for (i, &e) in v.iter().enumerate().skip(1) {
        match e {
            1 | 2 | 3 => {
                if dp[i - 1].1 <= e {
                    dp[i] = (max(dp[i - 1].0, 1), dp[i - 1].1)
                } else {
                    //
                }
            }
            0 => {
                //
            }
            _ => {
                //
            }
        }
    }
    dp[dp.len() - 1].0
}
