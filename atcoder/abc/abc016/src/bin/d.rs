#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: (i32, i32),
        b: (i32, i32),
        n: i32,
        mut points: [(i32, i32); n]
    }
    points.push(points[0]);
    println!(
        "{}",
        points
            .windows(2)
            .filter(|v| is_cross(a, b, v[0], v[1]))
            .count()
            / 2
            + 1
    )
}

#[inline]
fn is_cross(a: (i32, i32), b: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> bool {
    let d1 = direction(p1, p2, a);
    let d2 = direction(p1, p2, b);
    let d3 = direction(a, b, p1);
    let d4 = direction(a, b, p2);
    (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)))
        || (d1 == 0 && on_segment(p1, p2, a))
        || (d2 == 0 && on_segment(p1, p2, b))
        || (d3 == 0 && on_segment(a, b, p1))
        || (d4 == 0 && on_segment(a, b, p2))
}

#[inline]
fn direction(pi: (i32, i32), pj: (i32, i32), pk: (i32, i32)) -> i32 {
    let p1 = (pj.0 - pi.0, pj.1 - pi.1);
    let p2 = (pk.0 - pi.0, pk.1 - pi.1);
    p1.0 * p2.1 - p1.1 * p2.0
}

#[inline]
fn on_segment(pi: (i32, i32), pj: (i32, i32), pk: (i32, i32)) -> bool {
    use std::cmp::max;
    use std::cmp::min;
    min(pi.0, pj.0) <= pk.0
        && pk.0 <= pk.0
        && pk.0 <= max(pi.0, pj.0)
        && min(pi.1, pj.1) <= pk.1
        && pk.1 <= pk.1
        && pk.1 <= max(pi.1, pj.1)
}
