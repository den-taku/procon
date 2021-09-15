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
        unsafe { *count.get_unchecked_mut(v) += 1 }
    }
    let mut ans = 0;
    for i in 0..p {
        for j in i + 1..p {
            unsafe {
                ans += *count.get_unchecked(i)
                    * *count.get_unchecked(j)
                    * count.get_unchecked((i + j) % p)
            }
        }
    }
    ans *= 2;
    for i in 0..p {
        unsafe {
            ans +=
                *count.get_unchecked(i) * count.get_unchecked(i) * *count.get_unchecked((i + i) % p)
        }
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
