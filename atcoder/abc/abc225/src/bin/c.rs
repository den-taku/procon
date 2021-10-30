#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: [usize; n * m]
    }
    let fj = (b[0] - 1) % 7;
    if fj + m > 7 {
        println!("No");
        return;
    }
    let mut first = b[0];
    let mut next = b[0];
    for i in 0..n {
        for j in 0..m {
            if b[i * m + j] != next {
                println!("No");
                return;
            }
            next += 1;
        }
        first += 7;
        next = first;
    }
    println!("Yes")
}
