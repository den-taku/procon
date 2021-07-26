use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u128; n]
    }
    let mut product = 1;
    if a.iter().any(|&e| e == 0) {
        println!("0");
        return;
    }
    for e in a {
        product *= e;
        if product > 1_000_000_000_000_000_000 {
            println!("-1");
            return;
        }
    }
    println!("{}", product);
}
