use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        data: [(String, i32); n]
    }
    let mut data: Vec<_> = data
        .iter()
        .cloned()
        .enumerate()
        .map(|e| ((e.1).0, -(e.1).1, e.0))
        .collect();
    data.sort();
    for (_, _, i) in data {
        println!("{}", i + 1);
    }
}
