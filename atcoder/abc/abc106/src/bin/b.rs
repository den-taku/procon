use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize
    }
    let mut count = 0;
    for i in 1..n + 1 {
        let mut sum = 0;
        if i % 2 == 0 {
            continue;
        }
        for j in 1..i + 1 {
            if i % j == 0 {
                sum += 1;
            }
            if sum > 8 {
                break;
            }
        }
        if sum == 8 {
            count += 1;
        }
    }
    println!("{}", count);
}
