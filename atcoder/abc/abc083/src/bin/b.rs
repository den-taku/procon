use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        let s1 = i % 10;
        if s1 <= b {
            let s2 = ((i - s1) % 100) / 10 + s1;
            if s2 <= b {
                let s3 = ((i - s2) % 1000) / 100 + s2;
                if s3 <= b {
                    let s4 = ((i - s3) % 10000) / 1000 + s3;
                    if s4 <= b {
                        let s5 = ((i - s4) % 100000) / 10000 + s4;
                        if a <= s5 && s5 <= b {
                            ans += i;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans)
}
