#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i32; n],
        t: [i32; m]
    }
    let s = s.iter().map(|&c| c == 1).collect::<Vec<_>>();
    let t = t.iter().map(|&c| c == 1).collect::<Vec<_>>();
    let sset = s.iter().collect::<std::collections::HashSet<_>>();
    let tset = t.iter().collect::<std::collections::HashSet<_>>();
    if (t.iter().all(|&e| e) && s.iter().all(|&e| !e))
        || (t.iter().all(|&e| !e) && s.iter().all(|&e| e))
        || sset.len() < tset.len()
    {
        println!("-1")
    } else {
        let mut nearist = None;
        let mut now = s[0];
        let mut ans = 0;
        for a in t {
            if now != a {
                if nearist.is_some() {
                    ans += 2;
                    now = a;
                } else {
                    let dis = find_nearist(&s);
                    ans += dis + 1;
                    nearist = Some(dis);
                    now = a;
                }
            } else {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}

#[inline]
fn find_nearist(s: &[bool]) -> usize {
    let len = s.len();
    let mut left = len - 1;
    let mut right = 1;
    let mut count = 1 % len;
    while s[0] == s[left as usize] && s[0] == s[right as usize] {
        left -= 1;
        left %= len;
        right += 1;
        right %= len;
        count += 1;
    }
    count
}
