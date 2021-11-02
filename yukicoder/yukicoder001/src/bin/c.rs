// No.361 (https://yukicoder.me/problems/no/361)
// 門松ゲーム2
#![allow(unreachable_code)]

fn main() {
    let (l, d) = input();
    let mut dp = std::collections::HashMap::new();
    if grundy(l, d, &mut dp) == 0 {
        println!("matsu")
    } else {
        println!("kado")
    }
}

fn grundy(l: usize, d: usize, dp: &mut std::collections::HashMap<usize, usize>) -> usize {
    if let Some(&v) = dp.get(&l) {
        v
    } else {
        let mut set = std::collections::HashSet::new();
        for i in 1..l {
            for j in i + 1..l {
                if i + j >= l {
                    break;
                }
                let k = l - i - j;
                if k <= j {
                    break;
                }
                if k - i > d {
                    continue;
                }
                set.insert(grundy(i, d, dp) ^ grundy(j, d, dp) ^ grundy(k, d, dp));
            }
        }
        let mut rt = 0;
        while set.contains(&rt) {
            rt += 1
        }
        dp.insert(l, rt);
        rt
    }
}

#[inline(always)]
fn input() -> (usize, usize) {
    let e = read_line::<usize>();
    (e[0], e[1])
}

#[inline(always)]
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
