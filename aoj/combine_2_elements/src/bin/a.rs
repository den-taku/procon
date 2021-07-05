//use proconio::{fastout, input};
use std::fmt::Debug;
use std::str::FromStr;

// #[fastout]
fn main() {
    // input! {
    //     n: usize,
    //     t: [[i128; 50];50]
    // }
    let n = read_line::<usize>()[0];
    let mut t = Vec::with_capacity(n);
    let _ = (0..n).map(|_| {
        let elem = read_line::<i64>();
        t.push(elem);
    });
    t = t;
    unimplemented!()
}

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: i32,
    label: i32,
}

fn gabow_edmonds(gra: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let n = gra.len();
    let mut g: Vec<Vec<Edge>> = vec![vec![]; n + 1];
    let mut edges = Vec::new();
    let mut cnt = n + 1;
    for i in 0..n {
        for &to in &gra[i] {
            if i < to as usize {
                g[to as usize + 1].push(Edge {
                    to: i as i32 + 1,
                    label: cnt as i32,
                });
                g[i + 1].push(Edge {
                    to: to + 1,
                    label: cnt as i32 + 1,
                });
                cnt += 1;
                edges.push(Edge {
                    to: i as i32 + 1,
                    label: to + 1,
                })
            }
        }
    }
    unimplemented!()
}
