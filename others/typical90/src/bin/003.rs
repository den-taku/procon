#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n - 1]
    }
    let mut adj = vec![Vec::new(); n];
    for &(x, y) in &edges {
        adj[x - 1].push(y - 1);
        adj[y - 1].push(x - 1);
    }
    let mut dis = vec![None; n];
    dis[0] = Some(0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        let d = dis[v].unwrap() + 1;
        for &e in &adj[v] {
            if dis[e].is_none() {
                dis[e] = Some(d);
                queue.push_back(e);
            }
        }
    }
    let max_index = dis
        .iter()
        .enumerate()
        .max_by_key(|(_i, &e)| e.unwrap())
        .unwrap()
        .0;
    let mut dis = vec![None; n];
    dis[max_index] = Some(1);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(max_index);
    while let Some(v) = queue.pop_front() {
        let d = dis[v].unwrap() + 1;
        for &e in &adj[v] {
            if dis[e].is_none() {
                dis[e] = Some(d);
                queue.push_back(e);
            }
        }
    }
    println!("{}", dis.iter().map(|e| e.unwrap()).max().unwrap())
}
