use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n]
    }
    let cout = n / 2;
    println!(
        "{}",
        if x - a.iter().sum::<i32>() + cout as i32 >= 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
