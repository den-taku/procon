use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u32,
        drinks: [(u32, u32); n]
    }
    let mut sum = 0u32;
    for (i, &(volume, percent)) in drinks.iter().enumerate() {
        sum += volume * percent;
        if sum > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
