use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize
    }
    println!("{}", {
        let mut all = 360;
        while !(all % x == 0) {
            all += 360
        }
        all / x
    })
}
