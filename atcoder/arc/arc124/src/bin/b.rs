use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ar: [u128; n],
        br: [u128; n]
    }
    let mut ans = Vec::new();
    for i in 0..n {
        if ok(ar[0] ^ br[i], &ar, &br, n) {
            ans.push(ar[0] ^ br[i]);
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
    for e in ans {
        println!("{}", e);
    }
}

#[inline]
fn ok(value: u128, ar: &[u128], br: &[u128], n: usize) -> bool {
    let mut used = vec![false; n];
    for &a in ar {
        let mut flag = false;
        for (i, &b) in br.iter().enumerate() {
            if !used[i] && (a ^ b) == value {
                used[i] = true;
                flag = true;
                break;
            }
        }
        if !flag {
            return false;
        }
    }
    true
}
