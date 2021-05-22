use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut t = "".to_string();
    for e in s.chars() {
        match e {
            '0' => t = t + "0",
            '1' => t = t + "1",
            '6' => t = t + "9",
            '8' => t = t + "8",
            '9' => t = t + "6",
            _ => unreachable!(),
        }
    }
    for e in t.chars().rev() {
        print!("{}", e);
    }
}
