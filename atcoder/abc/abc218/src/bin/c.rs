#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String; n],
        ts: [String; n]
    }
    let s = ss.iter().map(|s| s.chars()).flatten().collect::<Vec<_>>();
    let mut t = ts.iter().map(|s| s.chars()).flatten().collect::<Vec<_>>();
    let mut mini = n;
    let mut maxi = 0;
    let mut minj = n;
    let mut maxj = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i * n + j] == '#' {
                mini = min(i, mini);
                maxi = max(i, maxi);
                minj = min(j, minj);
                maxj = max(j, maxj);
            }
        }
    }
    let s = ss
        .iter()
        .skip(mini)
        .take(maxi - mini + 1)
        .map(|v| v.chars().skip(minj).take(maxj - minj + 1))
        .flatten()
        .collect::<Vec<_>>();
    // println!("{:?}", s);
    for _ in 0..4 {
        let mut tmini = n;
        let mut tmaxi = 0;
        let mut tminj = n;
        let mut tmaxj = 0;
        for i in 0..n {
            for j in 0..n {
                if t[i * n + j] == '#' {
                    tmini = min(i, tmini);
                    tmaxi = max(i, tmaxi);
                    tminj = min(j, tminj);
                    tmaxj = max(j, tmaxj);
                }
            }
        }
        if (maxi - mini == tmaxi - tmini) && (maxj - minj == tmaxj - tminj) {
            let mut tmp = Vec::new();
            for i in tmini..tmaxi + 1 {
                for j in tminj..tmaxj + 1 {
                    tmp.push(t[i * n + j]);
                }
            }
            // let t = ts
            //     .iter()
            //     .skip(tmini)
            //     .take(tmaxi - tmini + 1)
            //     .map(|v| v.chars().skip(tminj).take(tmaxj - tminj + 1))
            //     .flatten()
            //     .collect::<Vec<_>>();
            // println!("{:?}", tmp);
            if s == tmp {
                println!("Yes");
                return;
            }
        }
        let mut tmp = vec!['#'; n * n];
        for i in 0..n {
            for j in 0..n {
                tmp[i * n + j] = t[(n - j - 1) * n + i];
            }
        }
        t = tmp;
    }

    println!("No");
}
