#![allow(unreachable_code)]
use proconio::fastout;
use std::boxed::Box;

#[fastout]
fn main() {
    let (_n, _ks, _a_s) = input();
}

#[inline(always)]
fn input() -> (usize, Vec<usize>, Vec<Vec<isize>>) {
    let (n, m) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    let mut ks = Vec::new();
    let mut a_s = Vec::new();
    for _ in 0..m {
        ks.push(read_line::<usize>()[0]);
        a_s.push(read_line::<isize>());
    }
    (n, ks, a_s)
}

#[inline(always)]
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
