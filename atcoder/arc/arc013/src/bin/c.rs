#![allow(unreachable_code)]
use proconio::fastout;

#[fastout]
fn main() {
    let n = read_line::<usize>()[0];
    let mut bit = 0usize;
    for _ in 0..n {
        let (x, y, z) = {
            let e = read_line::<usize>();
            (e[0], e[1], e[2])
        };
        let m = read_line::<usize>()[0];
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let mut zs = Vec::new();
        for _ in 0..m {
            let (a, b, c) = {
                let e = read_line::<usize>();
                (e[0], e[1], e[2])
            };
            xs.push(a);
            ys.push(b);
            zs.push(c);
        }
        xs.sort();
        ys.sort();
        zs.sort();
        for (last, v) in vec![x, y, z].into_iter().zip(vec![xs, ys, zs]) {
            let mut index = 1;
            for e in v {
                bit ^= e - index;
                index = e + 1;
            }
            bit ^= last - index;
        }
    }
    if bit != 0 {
        println!("WIN")
    } else {
        println!("LOSE")
    }
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
