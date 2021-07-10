use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut max = 0;
    let mut sum = 0;
    for c in s.chars() {
        if !(c == 'A' || c == 'C' || c == 'G' || c == 'T') {
            max = std::cmp::max(max, sum);
            sum = 0;
        } else {
            sum += 1;
        }
    }
    max = std::cmp::max(max, sum);
    println!("{}", max);
}
