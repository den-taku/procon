#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut a: [u64; n]
    }
    if n == 1 {
        let all = a[0] * (a[0] + 1) / 2;
        if a[0] >= k {
            println!("{}", all - (a[0] - k) * (a[0] - k + 1) / 2)
        } else {
            println!("{}", all);
        }
        return;
    }
    a.sort();
    a.reverse();
    a.push(0);
    let mut count = 1;
    let mut ans = 0;
    for (a, b) in a.windows(2).map(|v| (v[0], v[1])) {
        let res = a - b;
        let sum_a = a * (a + 1) / 2;
        let sum_b = b * (b + 1) / 2;
        if res * count < k {
            ans += count * (sum_a - sum_b);
            k -= res * count;
            count += 1;
        } else if res * count == k {
            ans += count * (sum_a - sum_b);
            break;
        } else {
            let tate = k / count;
            let yoko = k % count;
            ans += sum_a * count;
            ans -= (a - tate) * (a - tate + 1) / 2 * count;
            ans += yoko * (a - tate);
            break;
        }
    }
    println!("{}", ans);
}
