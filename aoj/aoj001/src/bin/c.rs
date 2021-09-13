// #![allow(unreachable_code)]
// //use proconio::{fastout, input};

// // #[fastout]
// fn main() {
//     // input! {
//     //     n: usize,
//     //     t: [[i128; 50];50]
//     // }
//     let n = read_line::<usize>()[0];
//     let mut t = Vec::with_capacity(n);
//     let _ = (0..n)
//         .map(|_| {
//             let elem = read_line::<i64>();
//             t.push(elem);
//         })
//         .collect::<Vec<()>>();
//     let _t = t;
//     unimplemented!()
// }

// #[inline]
// fn read_line<T>() -> Vec<T>
// where
//     T: std::str::FromStr,
//     <T as std::str::FromStr>::Err: std::fmt::Debug,
// {
//     let mut s = String::new();
//     std::io::stdin().read_line(&mut s).unwrap();
//     s.trim()
//         .split_whitespace()
//         .map(|c| T::from_str(c).unwrap())
//         .collect()
// }

fn main() {
    let v = input();
    let mut ans = Vec::new();
    'out: for i in 1..v.iter().min().unwrap() + 1 {
        for &e in &v {
            if e % i != 0 {
                continue 'out;
            }
        }
        ans.push(format!("{}\n", i))
    }
    println!("{}", ans.join(" "))
}

#[inline]
fn input() -> Vec<u32> {
    let _n = read_line::<usize>()[0];
    read_line()
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
