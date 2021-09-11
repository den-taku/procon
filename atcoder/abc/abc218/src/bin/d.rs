// O(n^3)
#![allow(unreachable_code)]
#![allow(dead_code)]
use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut points: [(usize, usize); n]
    }
    points.sort();
    let set = points.iter().collect::<std::collections::HashSet<_>>();
    let mut ans = 0;
    'out: for i in 0..n {
        for j in i + 1..n {
            if points[i].0 == points[j].0 {
                for k in j + 1..n {
                    if points[i].1 == points[k].1 && set.contains(&(points[k].0, points[j].1)) {
                        ans += 1;
                    }
                }
            } else {
                continue 'out;
            }
        }
    }
    println!("{}", ans);
}
