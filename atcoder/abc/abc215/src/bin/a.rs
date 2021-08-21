#![allow(unreachable_code)]
use proconio::fastout;

#[fastout]
fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    println!("{}", if s == "Hello,World!\n" { "AC" } else { "WA" })
}
