use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64
    }
    if 2 * n >= m {
        println!("{}", m / 2)
    } else {
        let res = m - 2 * n;
        println!("{}", n + res / 4);
    }
}
