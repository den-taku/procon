use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i128,
        mut money: [(i128, i128);n]
    }
    money.sort();
    let mut count = k;
    // let max = 10i64.pow(100u32) as i128;
    if k < money[0].0 {
        println!("{}", k);
        return;
    }
    for i in 0..n - 1 {
        count += money[i].1;
        // if count >= max {
        // println!("{}", max + 1);
        // return;
        // }
        if count < money[i + 1].0 {
            println!("{}", count);
            return;
        }
    }
    count += money[n - 1].1;
    println!("{}", count);
}
