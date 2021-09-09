#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        r: f64,
        positions: [(f64, f64); n]
    }
    let mut set = HashSet::new();
    for (mut a, mut b) in positions {
        a += 1e6;
        b += 1e6;
        set.insert(((a / r) as isize, (b / r) as isize));
    }
    let mut ans = 0;
    let mut visited = HashSet::new();
    while !set.is_empty() {
        ans += 1;
        let &(x, y) = set.iter().next().unwrap();
        set.remove(&(x, y));
        visited.insert((x, y));
        bfs(x, y, &mut set, &mut visited);
    }
    println!("{}", ans);
}

fn bfs(
    x: isize,
    y: isize,
    set: &mut HashSet<(isize, isize)>,
    visited: &mut HashSet<(isize, isize)>,
) {
    let dxs = [-1, 0, 1, 1, 1, 0, -1, -1];
    let dys = [-1, -1, -1, 0, 1, 1, 1, 0];
    for (&dx, &dy) in dxs.iter().zip(dys.iter()) {
        let newx = x + dx;
        let newy = y + dy;
        if !visited.contains(&(newx, newy)) && set.contains(&(newx, newy)) {
            set.remove(&(newx, newy));
            visited.insert((newx, newy));
            bfs(newx, newy, set, visited);
        }
    }
}
