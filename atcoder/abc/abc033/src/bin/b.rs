use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        towns: [(String, i32); n]
    }
    let mut sum = 0;
    let mut max = 0;
    for (i, (_, num)) in towns.iter().cloned().enumerate() {
        if towns[max].1 < num {
            max = i;
        }
        sum += num;
    }
    println!(
        "{}",
        if sum / 2 < towns[max].1 {
            towns[max].0.clone()
        } else {
            "atcoder".to_string()
        }
    )
}
