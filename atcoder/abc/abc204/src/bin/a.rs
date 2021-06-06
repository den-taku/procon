use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64
    }
    println!(
        "{}",
        if x == y {
            x
        } else if x == 0 {
            if y == 1 {
                2
            } else {
                1
            }
        } else if x == 1 {
            if y == 0 {
                2
            } else {
                0
            }
        } else {
            if y == 0 {
                1
            } else {
                0
            }
        }
    )
}
