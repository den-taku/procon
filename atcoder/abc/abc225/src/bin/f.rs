#![allow(unreachable_code)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ss: [String; n]
    }
    // let mut ans = String::new();
    // for i in 0..k {
    //     ans.push_str(&ss[i]);
    // }
    // for ss in ss.iter().permutations(k) {
    //     let mut tmp = String::new();
    //     for s in ss {
    //         tmp.push_str(&s);
    //     }
    //     ans = std::cmp::min(ans, tmp);
    // }
    // println!("{}", ans);
    let mut ss = ss
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    ss.sort();
    let mut ss = ss.into_iter().collect::<std::collections::VecDeque<_>>();
    let mut ans = String::new();
    let mut count = 0;
    while count < k && ss.len() > 2 {
        let one = ss.pop_front().unwrap();
        let mut two = ss.pop_front().unwrap();
        if one.len() <= two.len() {
            ans.push_str(&one.into_iter().collect::<String>());
            ss.push_front(two);
        } else {
            let mut flag = true;
            for i in 0..one.len() {
                if one[i] != two[i] {
                    ans.push_str(&one.iter().copied().collect::<String>());
                    flag = false;
                    break;
                }
            }
            if flag && two[one.len()] < one[0] {
                ans.push_str(&two.iter().copied().collect::<String>());
                two = one;
            } else if flag {
                ans.push_str(&one.iter().copied().collect::<String>());
            }
            ss.push_front(two);
        }
        count += 1;
    }
    if count != k {
        ans.push_str(&(ss.pop_front().unwrap().into_iter().collect::<String>()))
    }
    println!("{}", ans);
}
