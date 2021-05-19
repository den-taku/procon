use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let sum = make_cumulative_sum(&s);
    if s.len() % 2 == 1 {
        for i in 0..(s.len() + 1) / 2 {
            let all = sum[s.len() - i] - sum[i];
            if all == 0 || all == s.len() as i32 - 2 * (i as i32) {
                println!("{}", s.len() - i);
                return;
            }
        }
        println!("{}", s.len() / 2 + 1);
    } else {
        for i in 0..(s.len()) / 2 {
            let all = sum[s.len() - i] - sum[i];
            if all == 0 || all == s.len() as i32 - 2 * (i as i32) {
                println!("{}", s.len() - i);
               return;
            }
        }
        println!("{}", s.len() / 2);
    }
    
}

fn make_cumulative_sum(s: &str) -> Vec<i32> {
    let mut v = Vec::new();
    v.push(0);
    for c in s.chars() {
        match c {
            '0' => v.push(*v.last().unwrap()),
            '1' => v.push(*v.last().unwrap() + 1),
            _ => unreachable!()
        }
    }
    v
}

/*
XXXXXXXXXXXXXXXXXXX
|→                ←|
こう見ていった時に，iを増やせば左右i分は自由に切り替えられるようになる
when i = 6
  XXXXXXXXXXXXXXXXX
  _________________
   ________________
     or
  _________________
   ________________
    _______________
  _________________
偶数回の反転で元どおり，奇数回の反転で反転
*/