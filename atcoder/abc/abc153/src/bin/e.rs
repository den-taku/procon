#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        n: usize,
        magics: [(usize, usize); n]
    }
    let mut dp = Vec::new();
    dp.push(Some(0));
    for i in 1..h {
        let mut est = None;
        for &(damage, cost) in &magics {
            if i >= damage {
                if let Some(s) = dp[dp.len() - damage] {
                    if let Some(e) = est {
                        if s + cost < e {
                            est = Some(s + cost)
                        }
                    } else {
                        est = Some(s + cost)
                    }
                }
            }
        }
        dp.push(est);
    }
    for i in h..h + magics.iter().max_by_key(|e| e.0).unwrap().0 + 1 {
        let mut est = None;
        for &(damage, cost) in &magics {
            if i >= damage {
                if let Some(s) = dp[dp.len() - damage] {
                    if let Some(e) = est {
                        if s + cost < e {
                            est = Some(s + cost);
                        }
                    } else {
                        est = Some(s + cost);
                    }
                }
            }
        }
        dp.push(est);
    }
    println!(
        "{}",
        dp.iter()
            .skip(h)
            .filter(|e| if let Some(_) = e { true } else { false })
            .map(|e| e.unwrap())
            .min()
            .unwrap()
    );
}
