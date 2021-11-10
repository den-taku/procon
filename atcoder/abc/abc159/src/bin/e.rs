#![allow(unreachable_code)]
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [String; h]
    }
    let s = s
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| (c as u8 - b'0') as usize)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let mut ans = std::usize::MAX;
    'out: for t in 0..2 << (h - 1) {
        let cells = {
            let mut count = 0;
            for i in 0..h - 1 {
                if t >> i & 1 == 1 {
                    count += 1;
                }
            }
            count + 1
        };
        let mut sums = vec![0; cells];
        let mut count = 0;
        for j in 0..w {
            let mut idx = 0;
            let mut flag = false;
            for i in 0..h {
                sums[idx] += s[i * w + j];
                if sums[idx] > k {
                    flag = true;
                    break;
                }
                if t >> i & 1 == 1 {
                    idx += 1;
                }
            }
            if flag {
                idx = 0;
                sums = vec![0; cells];
                count += 1;
                for i in 0..h {
                    sums[idx] += s[i * w + j];
                    if sums[idx] > k {
                        continue 'out;
                    }
                    if t >> i & 1 == 1 {
                        idx += 1;
                    }
                }
            }
        }
        ans = std::cmp::min(ans, count + cells - 1)
    }
    println!("{}", ans);
}
