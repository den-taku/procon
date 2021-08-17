use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: u64,
        y: u64
    }
    let mut count = 0;
    while x <= y {
        x *= 2;
        count += 1;
    }
    println!("{}", count);
}
