#![allow(unreachable_code)]
use proconio::input;

const A: u8 = b'a';

fn main() {
    input! {
        s: String,
        t: String
    }
    let mut s = s
        .chars()
        .map(|c| if c == '?' { None } else { Some(c as u8 - A) })
        .collect::<Vec<_>>();
    let t = t.chars().map(|c| c as u8 - A).collect::<Vec<_>>();
    let mut index = None;
    'out: for (i, v) in s.windows(t.len()).enumerate() {
        for (c, &m) in v.iter().zip(t.iter()) {
            if !c.is_none() && c.unwrap() != m {
                continue 'out;
            }
        }
        index = Some(i)
    }
    println!(
        "{}",
        if index.is_none() {
            "UNRESTORABLE".to_string()
        } else {
            let index = index.unwrap();
            for (i, &c) in t.iter().enumerate() {
                s[index + i] = Some(c)
            }
            s.iter()
                .map(|c| (c.unwrap_or(0) + A) as char)
                .collect::<String>()
        }
    );
}
