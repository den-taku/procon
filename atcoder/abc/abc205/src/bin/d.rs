#![allow(dead_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        k: [usize; q]
    }
    a.push(0);
    a.sort();
    let a = a;
    let mut count = vec![0; n + 1];
    count[0] = a[0];
    for (i, e) in a.iter().enumerate().skip(1) {
        count[i] = e - a[i - 1] - 1 + count[i - 1]
    }
    let mut ans = Vec::with_capacity(q);
    for k in k {
        ans.push(binary_search(k, &count, &a).to_string());
    }
    println!("{}", ans.join("\n"));
}

fn binary_search(k: usize, count: &[usize], a: &[usize]) -> usize {
    if count[count.len() - 1] < k {
        return k + count.len() - 1;
    }
    let mut ub = count.len();
    let mut lb = 0;
    while ub - lb > 1 {
        let est = (ub + lb) / 2;
        if k <= count[est] {
            ub = est
        } else {
            lb = est
        }
    }
    a[ub] + k - count[ub] - 1
}
