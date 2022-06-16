#![allow(unreachable_code)]

fn main() {
    let _n = input();
    unimplemented!()
}

#[inline(always)]
fn input() -> usize {
    read_line::<usize>()[0]
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
