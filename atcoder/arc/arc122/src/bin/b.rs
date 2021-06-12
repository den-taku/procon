use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    }
    a.sort();
    let sum = a.iter().fold(0, |i, e| i + e);
    let mut best = std::i64::MAX;
    let mut add = 0;
    for i in 0..n / 2 {
        add += a[i];
        let est = a[i + 1];
        best = std::cmp::min(best, (2 * (i + 1) as i64 - n as i64) * est + sum + add);
        println!("best: {}", best);
    }
    for i in n / 2..n {
        let est = a[i];
        add += a[i];
        best = std::cmp::min(best, (2 * (i + 1) as i64 - n as i64) * est + sum + add);
        println!("best: {}", best);
    }
    println!("{}", best as f64 / n as f64);
}
