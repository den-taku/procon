use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut sum = 0;
    for e in a {
        if e >= 10 {
            sum += (e - 10);
        }
    }
    println!("{}", sum);
}
