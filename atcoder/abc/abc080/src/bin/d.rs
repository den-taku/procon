#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        stc: [(usize, usize, usize); n]
    }
    let mut channels = vec![Vec::new(); c];
    for (s, t, c) in stc {
        channels[c - 1].push((s, t))
    }
    let mut count = [0; 100_001];
    for channel in channels.iter_mut() {
        if channel.is_empty() {
            continue;
        }
        if channel.len() == 1 {
            count[channel[0].0 - 1] += 1;
            count[channel[0].1] -= 1;
            continue;
        }
        channel.sort();
        for ((s1, t1), (s2, _t2)) in channel.windows(2).map(|v| (v[0], v[1])) {
            count[s1 - 1] += 1;
            if t1 != s2 {
                count[t1] -= 1;
            } else {
                count[s2 - 1] -= 1;
            }
        }
        count[channel[channel.len() - 1].1] -= 1;
    }
    let mut sum = 0;
    let mut max = 1;
    for c in count.iter() {
        sum += c;
        max = std::cmp::max(max, sum);
    }
    println!("{}", max);
}
