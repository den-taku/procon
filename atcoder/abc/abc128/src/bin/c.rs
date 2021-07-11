use proconio::{fastout, input};

#[fastout]
fn main() {
    let (n, m) = {
        let input = read_line::<usize>();
        (input[0], input[1])
    };
    let mut ks = vec![vec![]; m];
    for s in &mut ks {
        let input = read_line::<usize>();
        for j in 0..input[0] {
            s.push(input[j + 1])
        }
    }
    input! {
        p: [usize; m]
    }
    let mut ans = 0;
    for i in 0..1 << n {
        let mut flag = 1;
        for j in 0..m {
            let mut add = 0;
            for &s in &ks[j] {
                if i >> (s - 1) & 1 == 1 {
                    add += 1;
                }
            }
            if add % 2 != p[j] {
                flag = 0;
                break;
            }
        }
        ans += flag;
    }
    println!("{}", ans);
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
