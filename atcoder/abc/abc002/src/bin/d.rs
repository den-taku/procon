#![allow(unreachable_code)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        relations: [(usize, usize); m]
    }
    let relations = relations.iter().collect::<std::collections::HashSet<_>>();
    let mut ans = 0;
    'out: for i in 0..1 << n {
        for j in 0..n {
            for k in j + 1..n {
                if i >> j & 1 == 1
                    && i >> k & 1 == 1
                    && relations.get(&(j + 1, k + 1)).is_none()
                    && relations.get(&(k + 1, j + 1)).is_none()
                {
                    continue 'out;
                }
            }
        }
        let mut count = 0;
        let mut i = i;
        if i & 1 == 1 {
            count += 1;
        }
        for _ in 0..n - 1 {
            i >>= 1;
            if i & 1 == 1 {
                count += 1;
            }
        }
        ans = std::cmp::max(ans, count);
    }
    println!("{}", ans);
}
