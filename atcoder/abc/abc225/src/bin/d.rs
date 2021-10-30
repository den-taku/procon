#![allow(unreachable_code)]
use proconio::{fastout, input};
use Query::*;

#[fastout]
fn main() {
    let (n, queries) = input();
    let mut next = vec![None; n];
    let mut pre = vec![None; n];
    for query in queries {
        match query {
            One(x, y) => {
                next[x - 1] = Some(y);
                pre[y - 1] = Some(x);
            }
            Two(x, y) => {
                next[x - 1] = None;
                pre[y - 1] = None;
            }
            Three(z) => {
                let mut ans = std::collections::VecDeque::new();
                let mut now = z;
                ans.push_back(now.to_string());
                while let Some(v) = next[now - 1] {
                    ans.push_back(v.to_string());
                    now = v;
                }
                let mut now = z;
                while let Some(v) = pre[now - 1] {
                    ans.push_front(v.to_string());
                    now = v;
                }
                println!(
                    "{} {}",
                    ans.len(),
                    ans.into_iter().collect::<Vec<_>>().join(" ")
                );
            }
        }
    }
}

enum Query {
    One(usize, usize),
    Two(usize, usize),
    Three(usize),
}

fn input() -> (usize, Vec<Query>) {
    let (n, q) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    let mut v = Vec::new();
    for _ in 0..q {
        let e = read_line::<usize>();
        if e[0] == 1 {
            v.push(One(e[1], e[2]))
        } else if e[0] == 2 {
            v.push(Two(e[1], e[2]))
        } else {
            v.push(Three(e[1]))
        }
    }
    (n, v)
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
