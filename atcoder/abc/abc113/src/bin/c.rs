#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        py: [(u64, u64); m]
    }
    let mut map = std::collections::HashMap::new();
    for (p, y) in &py {
        map.entry(p).or_insert_with(Vec::new).push(*y);
    }
    for (_, v) in map.iter_mut() {
        v.sort();
    }
    let mut ans = Vec::new();
    for (p, y) in &py {
        ans.push(format!(
            "{:06}{:06}",
            p,
            map[p].binary_search(&y).unwrap() + 1
        ));
    }
    println!("{}", ans.join("\n"));
}
