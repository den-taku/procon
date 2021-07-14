fn main() {
    let n = read_line::<usize>()[0];
    let mut v = vec![1; n + 1];
    for i in 2..n + 1 {
        v[i] = v[i - 1] + v[i - 2]
    }
    println!("{}", v[n]);
}

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
