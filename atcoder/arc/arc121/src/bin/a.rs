use proconio::{fastout, input};
use std::cmp::Reverse;

#[fastout]
fn main() {
    input! {
        n: usize,
        house_in: [(i64, i64);n]
    }
    let mut house = vec![(0, 0, 0); n];
    for i in 0..n {
        house[i] = (house_in[i].0, house_in[i].1, i);
    }

    house.sort();
    let first_x = (house[0].0 - house[n - 1].0).abs();
    let pointx1 = (house[0].2, house[n - 1].2);
    let s1x = (house[0].0 - house[n - 2].0).abs();
    let s2x = (house[1].0 - house[n - 1].0).abs();
    let s3x = (house[0].0 - house[n - 3].0).abs();
    let s4x = (house[1].0 - house[n - 2].0).abs();
    let s5x = (house[2].0 - house[n - 1].0).abs();
    let mut six_x = vec![
        (first_x, pointx1),
        (s1x, (house[0].2, house[n - 2].2)),
        (s2x, (house[1].2, house[n - 1].2)),
        (s3x, (house[0].2, house[n - 3].2)),
        (s4x, (house[1].2, house[n - 2].2)),
        (s5x, (house[2].2, house[n - 1].2)),
    ];

    house.sort_by_key(|e| e.1);
    let first_y = (house[0].1 - house[n - 1].1).abs();
    let pointy1 = (house[0].2, house[n - 1].2);
    let s1y = (house[0].1 - house[n - 2].1).abs();
    let s2y = (house[1].1 - house[n - 1].1).abs();
    let s3y = (house[0].1 - house[n - 3].1).abs();
    let s4y = (house[1].1 - house[n - 2].1).abs();
    let s5y = (house[2].1 - house[n - 1].1).abs();
    let mut six_y = vec![
        (first_y, pointy1),
        (s1y, (house[0].2, house[n - 2].2)),
        (s2y, (house[1].2, house[n - 1].2)),
        (s3y, (house[0].2, house[n - 3].2)),
        (s4y, (house[1].2, house[n - 2].2)),
        (s5y, (house[2].2, house[n - 1].2)),
    ];

    six_x.append(&mut six_y);
    six_x.sort_by_key(|&e| Reverse(e));
    if six_x[0].1 != six_x[1].1 {
        println!("{}", six_x[1].0);
    } else {
        println!("{}", six_x[2].0);
    }
}
