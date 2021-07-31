#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String
    }
    if s == "1" {
        println!("{}", 10u64.pow(10u32) * 2);
        return;
    } else if s == "0" {
        println!("{}", 10u64.pow(10u32));
        return;
    } else if s.starts_with("11") {
        let mut count = 0;
        for (c, (x, i)) in s.chars().zip([('1', 0), ('1', 1), ('0', 2)].iter().cycle()) {
            if c != *x {
                count = 0;
                break;
            } else if i == &0 {
                count += 1;
            }
        }
        if count > 0 {
            println!("{}", 10u64.pow(10u32) - count + 1);
            return;
        }
    } else if s.starts_with("10") {
        let mut count = 1;
        for (c, (i, x)) in s.chars().zip([('1', 1), ('0', 2), ('1', 0)].iter().cycle()) {
            if c != *i {
                count = 0;
                break;
            } else if x == &0 {
                count += 1;
            }
        }
        if count > 0 {
            println!("{}", 10u64.pow(10u32) - count + 1);
            return;
        }
    } else if s.starts_with("01") {
        let mut count = 1;
        for (c, (i, x)) in s.chars().zip([('0', 2), ('1', 0), ('1', 1)].iter().cycle()) {
            if c != *i {
                count = 0;
                break;
            } else if x == &0 {
                count += 1;
            }
        }
        if count > 0 {
            println!("{}", 10u64.pow(10u32) - count + 1);
            return;
        }
    }
    println!("0");
}
