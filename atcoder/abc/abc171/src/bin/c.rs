use proconio::{fastout, input};

const A: u8 = 'a' as u8;

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut s = Vec::with_capacity(10);
    while !(n == 0) {
        n -= 1;
        s.push(n % 26);
        n /= 26;
    }
    for &c in s.iter().rev() {
        print!("{}", (A + c as u8) as char)
    }
    println!("")
}
