use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let sum: i64 = a.iter().sum();
    let mut count = 0;
    let mut index = 0;
    for (i, &e) in a.iter().enumerate() {
        if 2 * (count + e) >= sum {
            index = i;
            break;
        }
        count += e;
    }
    let mut res = sum - count - a[index];
    if count >= res {
        res += a[index];
    } else {
        count += a[index]
    }
    println!("{}", (res - count).abs());
}
