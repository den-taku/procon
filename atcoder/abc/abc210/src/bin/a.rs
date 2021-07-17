use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a: i32,
        x: i32,
        y: i32
    }
    let sum;
    if n > a {
        sum = (n - a) * y + a * x
    } else {
        sum = n * x
    }
    println!("{}", sum)
}
