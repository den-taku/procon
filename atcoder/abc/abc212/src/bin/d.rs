#![allow(unreachable_code)]
use proconio::fastout;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    let n = {
        let ele = read_line::<usize>();
        ele[0]
    };
    let mut query = Vec::with_capacity(n);
    for _ in 0..n {
        let input = read_line::<i128>();
        query.push(input);
    }
    let mut set = BinaryHeap::new();
    let mut count2 = 0;
    for e in query {
        match e[0] {
            1 => {
                set.push(Reverse(e[1] - count2));
            }
            2 => {
                count2 += e[1];
            }
            3 => {
                let Reverse(value) = set.pop().unwrap();
                println!("{}", value + count2);
            }
            _ => unreachable!(),
        }
    }
}

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
