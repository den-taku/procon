#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::new();
    let mut time = Vec::new();
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [usize; k]
        }
        v.push(a);
        time.push(t)
    }
    let mut set = std::collections::HashSet::new();
    add(n, &mut set, &v);
    println!("{}", set.iter().map(|waza| time[waza - 1]).sum::<usize>());
}

fn add(waza: usize, set: &mut std::collections::HashSet<usize>, v: &[Vec<usize>]) {
    set.insert(waza);
    for &e in &v[waza - 1] {
        if !set.contains(&e) {
            add(e, set, v)
        }
    }
}
