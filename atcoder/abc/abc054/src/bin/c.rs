#![allow(unreachable_code)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m]
    }
    let edges = edges
        .into_iter()
        .flat_map(|(x, y)| vec![(x, y), (y, x)].into_iter())
        .collect::<std::collections::HashSet<_>>();
    println!(
        "{}",
        (2..n + 1)
            .permutations(n - 1)
            .filter(|permutation| edges.contains(&(1, permutation[0]))
                && permutation
                    .windows(2)
                    .all(|v| edges.contains(&(v[0], v[1]))))
            .count()
    )
}
