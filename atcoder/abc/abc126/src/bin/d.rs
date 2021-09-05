#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, usize); n - 1]
    }
    let mut adjacent = vec![Vec::new(); n];
    for (u, v, w) in edges {
        adjacent[u - 1].push((v - 1, w));
        adjacent[v - 1].push((u - 1, w));
    }
    let mut color = vec![None; n];
    color[0] = Some(0);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    while let Some(i) = queue.pop_front() {
        for &(v, w) in &adjacent[i] {
            if color[v].is_none() {
                if w % 2 == 0 {
                    color[v] = Some(color[i].unwrap());
                } else {
                    color[v] = Some(color[i].unwrap() ^ 1);
                }
                queue.push_back(v);
            }
        }
    }
    println!(
        "{}",
        color
            .iter()
            .map(|v| format!("{}\n", v.unwrap()))
            .collect::<String>()
            .trim()
    );
}
