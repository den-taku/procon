#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut ans = vec![0; n];
    let map = compress(&ab);
    println!("{:?}", map);
    println!(
        "{}",
        ans.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn compress(ab: &[(usize, usize)]) -> std::collections::HashMap<usize, usize> {
    let mut v = Vec::new();
    let mut map = std::collections::HashMap::new();
    for (a, b) in ab.iter() {
        v.push(a);
        v.push(b);
    }
    v.sort();
    v.dedup();
    for &(a, b) in ab.iter() {
        if map.get(&a).is_none() {
            let mut ub = v.len() - 1;
            let mut lb = 0;
            while ub - lb > 0 {
                let est = (ub + lb) / 2;
                if v[est] > &a {
                    ub = est
                } else {
                    lb = est
                }
            }
            map.insert(a, lb);
        }
        if map.get(&b).is_none() {
            let mut ub = v.len() - 1;
            let mut lb = 0;
            while ub - lb > 0 {
                let est = (ub + lb) / 2;
                if v[est] > &b {
                    ub = est
                } else {
                    lb = est
                }
            }
            map.insert(b, lb);
        }
    }
    map
}
