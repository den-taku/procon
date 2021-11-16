use std::io::{stdout, Write};

fn main() {
    let n = read_line::<usize>()[0];
    let mut dist = vec![0usize; n];
    for i in 2..n + 1 {
        println!("? 1 {}", i);
        stdout().flush().unwrap();
        let d = read_line::<usize>()[0];
        dist[i - 1] = d;
    }
    let mut max = 0;
    let mut index = 0;
    for i in 0..n {
        if max < dist[i] {
            max = dist[i];
            index = i;
        }
    }
    let mut dist = vec![0usize; n];
    for i in 1..n + 1 {
        if i == index + 1 {
            continue;
        }
        println!("? {} {}", index, i);
        stdout().flush().unwrap();
        let d = read_line::<usize>()[0];
        dist[i - 1] = d;
    }
    println!("{}", dist.iter().max().unwrap())
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
