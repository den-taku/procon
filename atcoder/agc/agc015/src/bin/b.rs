use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut ans = 0;
    for (i, c) in s.chars().enumerate() {
        if c == 'U' {
            ans += s.len() - i - 1;
            ans += 2 * i;
        } else {
            ans += 2 * (s.len() - i - 1);
            ans += i;
        }
    }
    println!("{}", ans);
}
