use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        pos: [(f64, f64); n]
    }
    let mut ans = 0f64;
    for i in 0..n {
        for j in i + 1..n {
            ans += dis(pos[i], pos[j]);
        }
    }
    println!("{}", ans * bikkuri(n - 1) * 2.0f64 / bikkuri(n));
}

#[inline]
fn bikkuri(n: usize) -> f64 {
    if n == 0 {
        1 as f64
    } else {
        n as f64 * bikkuri(n - 1)
    }
}

#[inline]
fn dis(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)).sqrt()
}
