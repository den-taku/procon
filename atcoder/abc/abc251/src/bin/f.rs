#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      edges: [(usize, usize); m]
    }
    let mut adjaecnt = vec![vec![]; n];
    for (u, v) in edges {
        adjaecnt[u - 1].push(v - 1);
        adjaecnt[v - 1].push(u - 1);
    }

    let mut ans1 = vec![];
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, &mut ans1, &adjaecnt, &mut visited);
    for (u, v) in ans1 {
        println!("{} {}", u, v)
    }

    let mut ans2 = vec![];
    let mut ve = std::collections::VecDeque::new();
    ve.push_back(0);
    let mut visited = vec![false; n];
    visited[0] = true;
    while let Some(v) = ve.pop_front() {
        for &e in &adjaecnt[v] {
            if !visited[e] {
                ans2.push((v + 1, e + 1));
                visited[e] = true;
                ve.push_back(e)
            }
        }
    }
    for (u, v) in ans2 {
        println!("{} {}", u, v)
    }
}

fn dfs(v: usize, ans: &mut Vec<(usize, usize)>, adjaecnt: &[Vec<usize>], visited: &mut [bool]) {
    for &e in &adjaecnt[v] {
        if !visited[e] {
            ans.push((v + 1, e + 1));
            visited[e] = true;
            dfs(e, ans, adjaecnt, visited);
        }
    }
}
