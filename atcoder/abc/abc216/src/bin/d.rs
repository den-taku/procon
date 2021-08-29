#![allow(unreachable_code)]
use proconio::fastout;

#[fastout]
fn main() {
    let (n, ks, a_s) = input();
    let mut indexes = vec![0; ks.len()];
    let mut count = vec![None; n];
    let mut top = 0u64;
    let mut tmp = 0i128;
    let mut cand = std::collections::VecDeque::new();
    for (i, e) in a_s
        .iter()
        .enumerate()
        .filter(|(i, _)| ks[*i] > 0)
        .map(|(i, v)| (i, v[0]))
    {
        top += 1;
        if let Some(idx) = count[(e - 1) as usize] {
            indexes[idx] += 1;
            indexes[i] += 1;
            cand.push_back(idx);
            cand.push_back(i);
            count[(e - 1) as usize] = None;
            tmp -= 2;
        } else {
            count[(e - 1) as usize] = Some(i);
        }
    }
    let mut culm = (top as i128 + tmp) as u64;
    while let Some(cnd) = cand.pop_front() {
        if culm == top {
            break;
        } else {
            culm += 1;
            if indexes[cnd] == ks[cnd] {
                continue;
            }
            let e = a_s[cnd][indexes[cnd]];
            if let Some(idx) = count[(e - 1) as usize] {
                indexes[idx] += 1;
                indexes[cnd] += 1;
                cand.push_back(idx);
                cand.push_back(cnd);
                count[(e - 1) as usize] = None;
                culm -= 2;
            } else {
                count[(e - 1) as usize] = Some(cnd);
            }
        }
    }
    if indexes.iter().zip(ks.iter()).all(|e| e.0 == e.1) {
        println!("Yes")
    } else {
        println!("No")
    }
}

#[inline(always)]
fn input() -> (usize, Vec<usize>, Vec<Vec<isize>>) {
    let (n, m) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    let mut ks = Vec::new();
    let mut a_s = Vec::new();
    for _ in 0..m {
        ks.push(read_line::<usize>()[0]);
        a_s.push(read_line::<isize>());
    }
    (n, ks, a_s)
}

#[inline(always)]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}
