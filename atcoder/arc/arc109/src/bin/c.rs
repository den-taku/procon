use proconio::{input, fastout};
use Hand::*;

#[derive(Clone, Copy)]
enum Hand {
    S,
    P,
    R
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u32,
        s: String
    }
    let mut v = Vec::new();
    for c in s.as_str().chars() {
        v.push(convert(c));
    }
    let m = 2i128.pow(k);
    let mut mem = Vec::new();
    for i in (1..k+1).rev() {
        for j in 0..2i128.pow(k) {
            mem.push(judge(v[(2 * j + 1 % v.len() as i128) as usize], v[(2 * j + 2 % v.len() as i128) as usize]));
        }
        v = mem;
        mem = Vec::new();
    }
    match v[0] {
        S => println!("S"),
        R => println!("R"),
        P => println!("P"),
    }
}

fn convert(s: char) -> Hand {
    match s {
        'R' => R,
        'S' => S,
        'P' => P,
        _ => unreachable!()
    }
}

fn merge(v: Vec<Hand>) -> Hand {
    if v.len() == 1 {
        v[0]
    } else {
        let mut u = Vec::new();
        for i in 0..v.len()/2 {
            u.push(judge(v[2*i+1], v[2*1+2]));
        }
        merge(u)
    }
}

fn judge(l: Hand, r: Hand) -> Hand {
    match l {
        S => {
            match r {
                S => S,
                P => S,
                R => R,
            }
        },
        P => {
            match r {
                S => S,
                P => P,
                R => R,
            }
        },
        R => {
            match r {
                S => R,
                P => P,
                R => R
            }
        }
    }
}