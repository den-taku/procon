#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; n],
        edges: [(usize, usize); m]
    }
    let mut ele = (0..n).collect::<Vec<_>>();
    for &(x, y) in &edges {
        let ind_x = find(&ele, x - 1);
        let ind_y = find(&ele, y - 1);
        if ind_x == ind_y {
            continue;
        }
        a[min(ind_x, ind_y)] = a[ind_x] + a[ind_y];
        a[max(ind_x, ind_y)] = 0;
        b[min(ind_x, ind_y)] = b[ind_x] + b[ind_y];
        b[max(ind_x, ind_y)] = 0;
        ele[ind_x] = min(ind_x, ind_y);
        ele[ind_y] = min(ind_x, ind_y);
    }
    if a.iter().zip(b).all(|e| *e.0 == e.1) {
        println!("Yes")
    } else {
        println!("No")
    }
}

#[inline]
fn find(ele: &[usize], index: usize) -> usize {
    if ele[index] == index {
        index
    } else {
        find(ele, ele[index])
    }
}
