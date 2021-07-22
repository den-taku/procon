use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut c1 = 0i32;
    let mut c2 = 0;
    let mut c3 = 0;
    let mut sum = 0;
    while !(n == 0) {
        match (n % 10) % 3 {
            0 => {
                c3 += 1;
            }
            1 => {
                c1 += 1;
                sum += 1;
            }
            2 => {
                c2 += 1;
                sum += 2;
            }
            _ => unreachable!(),
        }
        n /= 10;
    }
    match sum % 3 {
        0 => {
            println!("0");
        }
        1 => {
            if c1 > 0 {
                if c1 == 1 && c2 == 0 && c3 == 0 {
                    println!("-1")
                } else {
                    println!("1")
                }
            } else if c2 > 1 {
                if c1 == 0 && c2 == 2 && c3 == 0 {
                    println!("-1")
                } else {
                    println!("2")
                }
            } else {
                println!("-1")
            }
        }
        2 => {
            if c2 > 0 {
                if c1 == 0 && c2 == 1 && c3 == 0 {
                    println!("-1")
                } else {
                    println!("1")
                }
            } else if c1 > 1 {
                if c1 == 2 && c2 == 0 && c3 == 0 {
                    println!("-1")
                } else {
                    println!("2")
                }
            } else {
                println!("-1")
            }
        }
        _ => unreachable!(),
    }
}
