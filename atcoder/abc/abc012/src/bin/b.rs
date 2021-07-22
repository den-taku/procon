use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let h = n / 3600;
    let h1 = h / 10;
    let h2 = h % 10;
    let m = (n - 3600 * h) / 60;
    let m1 = m / 10;
    let m2 = m % 10;
    let s = n - 3600 * h - 60 * m;
    let s1 = s / 10;
    let s2 = s % 10;
    println!("{}{}:{}{}:{}{}", h1, h2, m1, m2, s1, s2);
}
