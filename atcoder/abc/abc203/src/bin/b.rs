use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i128,
        k: i128
    }
    let ans = 100 * k * n * (n + 1) / 2 + k * (k + 1) * n / 2;
    println!("{}", ans);
}
