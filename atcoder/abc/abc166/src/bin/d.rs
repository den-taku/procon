use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64
    }
    let mut memo = std::collections::HashMap::new();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..1000 {
        for j in 0..i + 1 {
            let v1 = if let Some(&e) = memo.get(&i) {
                e
            } else {
                let tmp = (i as u64).pow(5u32);
                memo.insert(i, tmp);
                tmp
            };
            let v2 = if let Some(&e) = memo.get(&j) {
                e
            } else {
                let tmp = (j as u64).pow(5u32);
                memo.insert(j, tmp);
                tmp
            };
            if v1 - v2 == x {
                ans1 = i;
                ans2 = j as i128;
                break;
            } else if v1 + v2 == x {
                ans1 = i;
                ans2 = -(j as i128);
                break;
            }
        }
    }
    println!("{} {}", ans1, ans2);
}
