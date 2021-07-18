use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        ns: [i128; t]
    }
    for n in ns {
        println!("{}", calc(n));
    }
}

#[inline]
fn calc(mut n: i128) -> i128 {
    use std::cmp::max;
    let mut v = Vec::new();
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    let mut ans = 0;
    for e in v {
        match e {
            0 => {
                ans = max(ans, 4);
            },
            4 | 5 | 6 => {
                if ans == 1 {
                    //
                }
                ans = max(ans, 2);
            },
            7 | 8 | 9 => {
                ans = max(ans, 3);
            }
            _ => {
                ans = max(ans, 1);
            }
        }
    }
    ans
}
