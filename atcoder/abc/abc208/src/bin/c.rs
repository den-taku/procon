use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i128,
        k: i128,
        a: [i128; n]
    }
    let base = k / n;
    let res = k % n;
    let mut a: Vec<(usize, i128, bool)> =
        a.iter().enumerate().map(|e| (e.0, *e.1, false)).collect();
    a.sort_by_key(|e| e.1);
    for i in 0..res as usize {
        a[i].2 = true;
    }
    a.sort_by_key(|e| e.0);
    for &e in &a {
        if e.2 {
            println!("{}", base + 1);
        } else {
            println!("{}", base);
        }
    }
}
