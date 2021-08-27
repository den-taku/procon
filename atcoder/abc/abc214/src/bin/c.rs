#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [u64; n],
        mut t: [u64; n]
    }
    let mut visited = vec![false; n];
    for (i, mut time) in t.clone().iter().copied().enumerate() {
        if visited[i] && time >= t[i] {
            continue;
        }
        visited[i] = true;
        time += s[i];
        for j in 1..n {
            let index = (i + j) % n;
            if time < t[index] || (time == t[index] && !visited[index]) {
                visited[index] = true;
                t[index] = time;
                time += s[index];
            } else {
                break;
            }
        }
    }
    println!(
        "{}",
        t.iter()
            .map(|&n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
