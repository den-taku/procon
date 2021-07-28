use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: u32,
        a: [u32; n]
    }
    let mut end = 0;
    let mut sum = 0;
    for e in a {
        if end <= e {
            sum += t;
            end = e + t;
        } else {
            sum += t - (end - e);
            end = e + t;
        }
    }
    println!("{}", sum);
}
