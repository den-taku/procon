#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: String
    }
    let x = x
        .chars() /*.map(|c| c as u8 - b'0')*/
        .collect::<Vec<_>>();
    let a = x
        .iter()
        .copied()
        .take_while(|c| c != &'.')
        .collect::<Vec<_>>();
    let value: i64 = a.iter().copied().collect::<String>().parse().unwrap();
    if x[a.len() + 1] as u8 - b'0' >= 5 {
        println!("{}", value + 1)
    } else {
        println!("{}", value)
    }
}
