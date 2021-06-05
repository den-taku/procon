use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        devices: [(usize, usize); n]
    }
    let mut dp = HashMap::<(usize, usize, usize, usize), i32>::with_capacity(n * n * n * n);
    println!("{}", rec(w, h, (0, w + 1, 0, h + 1), &devices, &mut dp));
}

fn rec(
    w: usize,
    h: usize,
    (i, j, k, l): (usize, usize, usize, usize),
    devices: &[(usize, usize)],
    dp: &mut HashMap<(usize, usize, usize, usize), i32>,
) -> i32 {
    if let Some(v) = dp.get(&(i, j, k, l)) {
        *v
    } else {
        let mut best = 0;
        for &(x, y) in devices {
            if i < x && x < j && k < y && y < l {
                let est = rec(w, h, (i, x, y, l), devices, dp)
                    + rec(w, h, (x, j, y, l), devices, dp)
                    + rec(w, h, (i, x, k, y), devices, dp)
                    + rec(w, h, (x, j, k, y), devices, dp)
                    + (j - i - 1) as i32
                    + (l - k - 1) as i32
                    - 1;
                best = std::cmp::max(best, est);
            }
        }
        dp.insert((i, j, k, l), best);
        best
    }
}