use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32
    }
    let hoge = b - a + 1;
    println!("{}", if hoge > 0 { hoge } else { 0 });
}
