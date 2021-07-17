use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }
    let mut i = 0;
    for c in s.chars() {
        if c == '1' {
            break;
        }
        i += 1;
    }
    println!("{}", {
        if i % 2 == 0 {
            "Takahashi"
        } else {
            "Aoki"
        }
    })
}
