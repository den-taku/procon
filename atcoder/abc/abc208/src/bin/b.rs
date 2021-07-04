use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut p: usize
    }
    let a = [f(10), f(9), f(8), f(7), f(6), f(5), f(4), f(3), f(2), f(1)];
    let mut ans = 0;
    for i in 0..10 {
        if p % a[i] == 0 {
            ans += p / a[i];
            break;
        } else {
            ans += p / a[i];
            p %= a[i];
        }
    }
    println!("{}", ans);
}

fn f(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * f(n - 1)
    }
}
