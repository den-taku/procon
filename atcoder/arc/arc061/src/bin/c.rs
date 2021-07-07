use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: i64,
    }
    let mut ans = 0i64;
    let mut v = Vec::with_capacity(10);
    while s > 0 {
        v.push(s % 10);
        s /= 10;
    }
    let pow = [
        1,
        10,
        10 * 10,
        10i64.pow(3),
        10i64.pow(4),
        10i64.pow(5),
        10i64.pow(6),
        10i64.pow(7),
        10i64.pow(8),
        10i64.pow(9),
    ];
    for i in 0..1 << (v.len() - 1) {
        let mut k = 1;
        ans += v[0];
        for j in 0..v.len() - 1 {
            if i >> j & 1 == 1 {
                ans += v[j + 1];
                k = 1;
            } else {
                ans += v[j + 1] * pow[k];
                k += 1;
            }
        }
    }
    println!("{}", ans);
}
