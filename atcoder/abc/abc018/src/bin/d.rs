use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        q: usize,
        r: usize,
        chocolate: [(usize, usize, i64);r]
    }
    let mut ans = 0;
    for i in 0..m {
        let est = calulate(i, p, q, &chocolate);
        ans = if est > ans { est } else { ans };
    }
    println!("{}", ans);
}

fn calulate(mut index: usize, p: usize, q:usize, chocolate: &Vec<(usize, usize, i64)>) -> i64 {
    let mut girls = HashSet::with_capacity(p);
    let mut boys = HashSet::with_capacity(q);
    boys.insert(index);
    let mut i = 0;
    while i < p {
        let contains = assemble(index, &chocolate, &girls);
    }
    unimplemented!()
}

fn assemble(index: usize, chocolate: &Vec<(usize, usize, i64)>, girls: &HashSet<usize>) -> Vec<(i64, usize)> {
    let mut contains = Vec::new();
    for e in chocolate {
        if e.1 == index && !girls.contains(&e.0) {
            contains.push((e.2, e.0));
        }
    }
    contains.sort();
    contains
}