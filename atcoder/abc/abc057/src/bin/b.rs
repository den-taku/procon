#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        students: [(i64, i64); n],
        points: [(i64, i64); m]
    }
    for &(x, y) in &students {
        let mut index = 0;
        let mut distance = std::i64::MAX;
        for (i, &(a, b)) in points.iter().enumerate() {
            let dis = (x - a).abs() + (y - b).abs();
            if dis < distance {
                distance = dis;
                index = i + 1;
            }
        }
        println!("{}", index);
    }
}
