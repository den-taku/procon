use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let map = prime_factor(n);
    let mut ans = 0;
    for (&_, &e) in map.iter() {
        ans += 1;
        let mut i = 2;
        let mut e = e;
        while e > i {
            ans += 1;
            e -= i;
            i += 1;
        }
    }
    println!("{}", ans);
}

#[inline]
fn prime_factor(mut n: usize) -> std::collections::HashMap<usize, usize> {
    let mut map = std::collections::HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            if let Some(&e) = map.get(&i) {
                map.insert(i, e + 1);
            } else {
                map.insert(i, 1);
            }
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        map.insert(n, 1);
        map
    } else {
        map
    }
}
