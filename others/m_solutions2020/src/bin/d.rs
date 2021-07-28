use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut money = 1000;
    for (x, y) in a.windows(2).map(|s| (s[0], s[1])) {
        if x < y {
            money += money / x * (y - x)
        }
    }
    println!("{}", money);
}
