use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [i64; n]
    }
    let mut b = Vec::with_capacity(n);
    b.push(a_s[n - 1]);
    let mut c = Vec::with_capacity(n);
    for (i, &a) in a_s.iter().rev().skip(1).enumerate() {
        if b[i] >= a {
            b.push(a)
        } else {
            b.push(b[i])
        }
    }
    // println!("{:?}", b);
    for i in 0..n {
        c.push(a_s[i] - b[n - i - 1]);
    }
    // println!("{:?}", c);
    let b_sum = b.iter().fold(0i64, |e, s| {
        e + s.abs()
    });
    let c_sum = c.iter().fold(0i64, |e, s| {
        e + s.abs()
    });
    println!("{}", b_sum + c_sum);
}
