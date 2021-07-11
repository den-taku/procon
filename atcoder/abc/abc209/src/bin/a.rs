use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32
    }
    println!("{}", std::cmp::max(b - a + 1, 0));
}
