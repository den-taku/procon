use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i32,
        times: [i32; n]
    }
    let mut sum = times[0] + times[1] + times[2];
    if sum < k {
        println!("3");
        return;
    }
    for (i, &t) in times.iter().skip(3).enumerate() {
        sum -= times[i];
        sum += t;
        if sum < k {
            println!("{}", i + 4);
            return;
        }
    }
    println!("-1");
}
