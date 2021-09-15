#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: usize,
        n: usize
    }
    let mut count = vec![0; p];
    for i in 0..p {
        let v = pow(i, n, p);
        count[v] += 1;
    }
    let mut ans = 0;
    for i in 0..p {
        for j in i+1..p {
            ans += count[i] * count[j] * count[(i + j) % p];
        }
    }
    ans *= 2;
    for i in 0..p {
        ans += count[i] * count[i] * count[(i + i) % p];
    }
    println!("{}", ans);
}

#[inline]
fn pow(a: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else if n % 2 == 0 {
        let ret = pow(a, n / 2, p);
        ret * ret % p
    } else {
        let ret = pow(a, n / 2, p);
        ret * ret * a % p
    }
}
