use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64
    }
    if a >= b && (a - b) % 2 == 0 || a < b && (b - a) % 2 == 0 {
        println!("{}", (std::cmp::max(a, b) - std::cmp::min(a, b)) / 2)
    } else {
        println!("{}", {
            let res1 = std::cmp::min(a - 1, b - 1) + 1;
            let resn = std::cmp::min(n - a, n - b) + 1;
            (std::cmp::max(a, b) - std::cmp::min(a, b) - 1) / 2 + std::cmp::min(res1, resn)
        })
    }
}
