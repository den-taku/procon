//use proconio::{fastout, input};

// #[fastout]
fn main() {
    // input! {
    //     n: usize,
    //     t: [[i128; 50];50]
    // }
    let n = read_line::<usize>()[0];
    let mut t = Vec::with_capacity(n);
    let _ = (0..n).map(|_| {
        let elem = read_line::<i64>();
        t.push(elem);
    }).collect::<Vec<()>>;
    _t = t;
    unimplemented!()
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
