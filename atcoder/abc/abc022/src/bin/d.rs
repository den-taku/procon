#![allow(dead_code, clippy::float_cmp)]
#![allow(unreachable_code)]
// use graham_scan_library::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [(f64, f64); n],
        b: [(f64, f64); n]
    }
    let mut m_a = (0.0, 0.0);
    let mut m_b = (0.0, 0.0);
    for i in 0..n {
        m_a.0 += a[i].0;
        m_a.1 += a[i].1;
        m_b.0 += b[i].0;
        m_b.1 += b[i].1;
    }
    m_a.0 /= n as f64;
    m_b.0 /= n as f64;
    m_a.1 /= n as f64;
    m_b.1 /= n as f64;
    let mut la = 0.0;
    let mut lb = 0.0;
    for i in 0..n {
        let da = (a[i].0 - m_a.0) * (a[i].0 - m_a.0) + (a[i].1 - m_a.1) * (a[i].1 - m_a.1);
        if la < da {
            la = da;
        }
        let db = (b[i].0 - m_b.0) * (b[i].0 - m_b.0) + (b[i].1 - m_b.1) * (b[i].1 - m_b.1);
        if lb < db {
            lb = db;
        }
    }
    println!("{}", (lb / la).sqrt());
    // if n == 2 {
    //     let d1 =
    //         ((a[0].0 - a[1].0) * (a[0].0 - a[1].0) + (a[0].1 - a[1].1) * (a[0].1 - a[1].1)).sqrt();
    //     let d2 =
    //         ((b[0].0 - b[1].0) * (b[0].0 - b[1].0) + (b[0].1 - b[1].1) * (b[0].1 - b[1].1)).sqrt();
    //     println!("{}", d2 / d1);
    //     return;
    // }
    // let mut graham1 = GrahamScan::run(&a);
    // graham1.push(graham1[0]);
    // let mut graham2 = GrahamScan::run(&b);
    // graham2.push(graham2[0]);
    // let mut sum1 = 0.0;
    // for ((x1, y1, _), (x2, y2, _)) in graham1.windows(2).map(|v| (v[0], v[1])) {
    //     sum1 += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
    // }
    // let mut sum2 = 0.0;
    // for ((x1, y1, _), (x2, y2, _)) in graham2.windows(2).map(|v| (v[0], v[1])) {
    //     sum2 += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
    // }
    // println!("{}", sum2 / sum1);
}

pub mod graham_scan_library {
    pub struct GrahamScan {}

    impl GrahamScan {
        pub fn run(vertices: &[(f64, f64)]) -> Vec<(f64, f64, usize)> {
            let mut v = vertices
                .iter()
                .copied()
                .enumerate()
                .map(|e| ((e.1).0, (e.1).1, e.0))
                .collect::<Vec<_>>();
            v.sort_by(|b, a| {
                a.1.partial_cmp(&b.1)
                    .unwrap()
                    .then_with(|| b.0.partial_cmp(&a.0).unwrap())
            });
            let p0 = v.pop().unwrap();
            v.sort_by(|a, b| {
                if direction(p0.0, p0.1, a.0, a.1, b.0, b.1) > 0.0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            let mut indexes = vec![p0, v[0], v[1]];
            for &e in v.iter().skip(2) {
                while {
                    let top = indexes[indexes.len() - 1];
                    let next = indexes[indexes.len() - 2];
                    direction(next.0, next.1, top.0, top.1, e.0, e.1) < 0.0
                } {
                    indexes.pop();
                }
                indexes.push(e);
            }
            indexes
        }
    }

    fn direction(p0x: f64, p0y: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> f64 {
        let v1 = (p1x - p0x, p1y - p0y);
        let v2 = (p2x - p0x, p2y - p0y);
        v1.0 * v2.1 - v1.1 * v2.0
    }

    #[cfg(test)]
    mod tests_graham_scan {
        use super::*;

        #[test]
        fn for_graham_scan() {
            let input = [
                (
                    vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)],
                    vec![(0.0, 2.0), (2.0, 0.0), (-2.0, 0.0), (0.0, -2.0)],
                ),
                (
                    vec![
                        (3.0, 4.0),
                        (1.0, 3.0),
                        (4.0, 3.0),
                        (2.0, 2.0),
                        (0.0, 1.0),
                        (2.0, 0.0),
                    ],
                    vec![
                        (5.0, 5.),
                        (-1.0, 2.0),
                        (-1.0, -3.0),
                        (2.0, 1.0),
                        (2.0, 6.0),
                        (4.0, -3.0),
                    ],
                ),
            ];
            let ans = [2.8284271247, 2.2360679775];
            for ((a, b), ans) in input.iter().zip(ans) {
                let mut graham1 = GrahamScan::run(&a);
                graham1.push(graham1[0]);
                let mut graham2 = GrahamScan::run(&b);
                graham2.push(graham2[0]);
                let mut sum1 = 0.0;
                for ((x1, y1, _), (x2, y2, _)) in graham1.windows(2).map(|v| (v[0], v[1])) {
                    sum1 += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
                }
                let mut sum2 = 0.0;
                for ((x1, y1, _), (x2, y2, _)) in graham2.windows(2).map(|v| (v[0], v[1])) {
                    sum2 += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
                }
                assert!(sum2 / sum1 - ans < 1e-6);
            }
        }
    }
}
