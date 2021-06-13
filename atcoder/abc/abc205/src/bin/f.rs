use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        poss: [(usize, usize, usize, usize); n]
    }
    let mut dp = HashMap::<(usize, usize, usize, usize, i128), i64>::with_capacity(n * n * n * n);
    println!("{}", rec(0, h, w, (1, w, 1, h), &poss, &mut dp));
}

fn rec(
    set: i128,
    h: usize,
    w: usize,
    (i, j, k, l): (usize, usize, usize, usize),
    poss: &[(usize, usize, usize, usize)],
    dp: &mut HashMap<(usize, usize, usize, usize, i128), i64>
) -> i64 {
    if let Some(v) = dp.get(&(i, j, k, l, set)) {
        *v
    } else {
        let mut best = 0;
        for (i, &(x_l, y_l, x_u, y_u)) in poss.iter().enumerate() {
            //
        }
        unimplemented!()
    }
}