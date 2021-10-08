#![allow(dead_code)]
#![allow(unreachable_code)]

fn main() {
    let input = input();
    let n = input.len();
    let mut ans = 0;
    'out: for set in 1..1 << n {
        let mut people = vec![false; n];
        let mut count = 0;
        for i in 0..n {
            if set >> i & 1 == 1 {
                count += 1;
                people[i] = true;
            }
        }
        for i in 0..n {
            if set >> i & 1 == 1 {
                for &(j, p) in &input[i] {
                    if people[j - 1] == (p == 0) || !people[j - 1] == (p == 1) {
                        continue 'out;
                    }
                }
            }
        }
        ans = std::cmp::max(ans, count);
    }
    println!("{}", ans);
}

#[inline(always)]
fn input() -> Vec<Vec<(usize, usize)>> {
    let n = read_line::<usize>()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        let mut v = Vec::new();
        let s = read_line::<usize>()[0];
        for _ in 0..s {
            let e = read_line::<usize>();
            v.push((e[0], e[1]))
        }
        vec.push(v)
    }
    vec
}

#[inline]
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
