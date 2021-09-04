#![allow(unreachable_code)]
use proconio::fastout;
use std::collections::{BTreeSet, HashMap, VecDeque};
use Query::*;

#[fastout]
fn main() {
    let queries = input();
    let mut tree = BTreeSet::new();
    let mut next = BTreeSet::new();
    let mut array = VecDeque::new();
    let mut count = HashMap::<usize, usize>::new();
    let mut count_next = HashMap::<usize, usize>::new();
    let mut count_buffer = HashMap::<usize, usize>::new();
    for (i, query) in queries.into_iter().enumerate() {
        match query {
            One(x) => {
                array.push_back(x);
                if !next.insert(x) {
                    if let Some(&v) = count_next.get(&x) {
                        count_next.insert(x, v + 1);
                        count_buffer.insert(x, v + 1);
                    } else {
                        count_next.insert(x, 1);
                        count_buffer.insert(x, 1);
                    }
                }
            }
            Two => {
                if tree.len() == 0 {
                    let a = array.pop_front().unwrap();
                    println!("{}", a);
                    if !(next.len() == 0) {
                        if let Some(&v) = count_next.get(&a) {
                            if v == 1 {
                                count_next.remove(&a);
                                count_buffer.remove(&a);
                            } else {
                                count_next.insert(a, v - 1);
                                count_buffer.insert(a, v - 1);
                            }
                        } else {
                            next.remove(&a);
                        }
                    }
                } else {
                    let &e = tree.range(0..).next().unwrap();
                    println!("{}", e);
                    if let Some(&v) = count.get(&e) {
                        if v == 1 {
                            count.remove(&e);
                        } else {
                            count.insert(e, v - 1);
                        }
                        let &v = count_next.get(&e).unwrap();
                        if v == 1 {
                            count_next.remove(&e);
                            count_buffer.remove(&e);
                        } else {
                            count_next.insert(e, v - 1);
                            count_buffer.insert(e, v - 1);
                        }
                        tree.insert(e);
                    } else {
                        next.remove(&e);
                        tree.remove(&e);
                    }
                }
            }
            Three => {
                tree = &tree | &next;
                count = count_next;
                count_next = HashMap::new();
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
