#![allow(unreachable_code)]

fn main() {
    let problems = input();
    let mut ans = Vec::new();
    for p in problems {
        let (c1, c2) = solve(p);
        ans.push(format!("{} {}", c1, c2));
    }
    println!("{}", ans.join("\n"))
}

fn solve(n: usize) -> (usize, usize) {
    match n % 3 {
        0 => (n / 3, n / 3),
        1 => (n / 3 + 1, n / 3),
        2 => (n / 3, n / 3 + 1),
        _ => unreachable!(),
    }
}

#[inline(always)]
fn input() -> Vec<usize> {
    let t = read_line::<usize>()[0];
    let mut problems = vec![0; t];
    for p in problems.iter_mut() {
        *p = read_line::<usize>()[0];
    }
    problems
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
