use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let s = trans(&s);
    let mut sum = vec![0; s.len()+1];
    sum[0] += 0;
    for i in 1..s.len()+1 {
        sum[i] = sum[i - 1] + s[i-1];
    }
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

fn trans(s: &str) -> Vec<i32> {
    let mut v = Vec::new();
    for c in s.chars() {
        match c {
            '0' => v.push(0),
            '1' => v.push(1),
            _ => unreachable!(),
        }
    }
    v
}

// fn make_cumulative_sum(s: &str) -> Vec<i32> {
//     let mut v = Vec::new();
//     v.push(0);
//     for c in s.chars() {
//         match c {
//             '0' => v.push(*v.last().unwrap()),
//             '1' => v.push(*v.last().unwrap() + 1),
//             _ => unreachable!()
//         }
//     }
//     v
// }
