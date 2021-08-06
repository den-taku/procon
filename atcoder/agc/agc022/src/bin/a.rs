#![allow(unreachable_code)]
use proconio::{fastout, input};

const A: u8 = b'a';

#[fastout]
fn main() {
    input! {
        s: String
    }
    if s.len() == 26 {
        let mut u = s.chars().map(|c| c as u8 - A).collect::<Vec<_>>();
        let mut rest = Vec::new();
        for (i, (a, b)) in u.windows(2).map(|e| (e[0], e[1])).enumerate().rev() {
            if a < b {
                u = u.iter().take(i).copied().collect::<Vec<_>>();
                rest = s.chars().skip(i).map(|c| c as u8 - A).collect::<Vec<_>>();
                break;
            }
        }
        if u.len() == 26 {
            println!("-1");
            return;
        } else {
            for &e in &u {
                print!("{}", (e + A) as char);
            }
            let x = s.chars().nth(u.len()).unwrap() as u8 - A;
            rest.sort();
            for e in rest {
                if x < e {
                    println!("{}", (e + A) as char);
                    return;
                }
            }
        }
    } else {
        let mut set = std::collections::HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        for i in 0..26 {
            if !set.contains(&((A + i as u8) as char)) {
                print!("{}", s);
                println!("{}", (A + i) as char);
                return;
            }
        }
    }
}
