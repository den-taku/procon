#![allow(unreachable_code)]
use proconio::fastout;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use Query::*;

#[fastout]
fn main() {
    let queries = input();
    let mut heap = BinaryHeap::new();
    let mut array = VecDeque::new();
    for query in queries {
        match query {
            One(x) => {
                array.push_back(x);
            }
            Two => {
                if heap.len() == 0 {
                    let a = array.pop_front().unwrap();
                    println!("{}", a);
                } else {
                    let Reverse(e) = heap.pop().unwrap();
                    println!("{}", e);
                }
            }
            Three => {
                for e in array {
                    heap.push(Reverse(e));
                }
                array = VecDeque::new();
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Query {
    One(usize),
    Two,
    Three,
}

#[inline(always)]
fn input() -> Vec<Query> {
    let q = read_line::<usize>()[0];
    let mut v = Vec::new();
    for _ in 0..q {
        let e = read_line::<usize>();
        if e[0] == 1 {
            v.push(One(e[1]));
        } else if e[0] == 2 {
            v.push(Two);
        } else {
            v.push(Three);
        }
    }
    v
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
