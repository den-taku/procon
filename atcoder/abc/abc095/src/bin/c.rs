use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        mut x: i32,
        mut y: i32,
    }
    let mut sum = 0;
    if a + b >= 2 * c {
        let count = std::cmp::min(x, y);
        x -= count;
        y -= count;
        sum += 2 * count * c;
    }
    if a >= 2 * c {
        sum += 2 * x * c;
        x = 0;
        y = std::cmp::max(0, y - x);
    }
    if b >= 2 * c {
        sum += 2 * y * c;
        y = 0;
        x = std::cmp::max(0, x - y);
    }
    sum += x * a;
    sum += y * b;
    println!("{}", sum);
}
