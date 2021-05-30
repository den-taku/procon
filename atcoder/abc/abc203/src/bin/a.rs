use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if a == c {
        println!("{}", b);
    } else {
        println!("{}", 0);
    }
}
