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
    for i in 0..n-p+1 {
        // dfs: nCp
        let cand = dfs(n, m, p, q, i, HashSet::with_capacity(p), &chocolate);
        if cand > ans {
            ans = cand;
        }
    }
    println!("{}", ans);
}

fn dfs(n: usize, m: usize, p: usize, q: usize, index: usize, mut candidate: HashSet<usize>, chocolate: &Vec<(usize, usize, i64)>) -> i64 {
    // dfs: start from index
    let candidate = {
        candidate.insert(index);
        candidate
    };
    // get nCp, then calculate in greedy
    if candidate.len() == p {
        return calculate(n, m, q, candidate, chocolate);
    }
    let mut best = 0;
    for i in index+1..n {
        let cand = dfs(n, m, p, q, i, candidate.clone(), chocolate);
        if best < cand {
            best = cand;
        }
    }
    return best;
}

fn calculate(_n: usize, m: usize, q: usize, candidate: HashSet<usize>, chocolate: &Vec<(usize, usize, i64)>) -> i64 {
    let mut count = vec![0; m];
    for e in chocolate {
        if candidate.contains(&(e.0 - 1)) {
            count[e.1 - 1] += e.2;
        }
    }
    count.sort();
    let mut est = 0;
    // greedy
    for (i, e) in count.iter().rev().enumerate() {
        if i == q {
            break;
        }
        est += e;
    }
    est
}
