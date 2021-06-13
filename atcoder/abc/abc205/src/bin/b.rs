use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut array = vec![0; n];
    for e in &a {
        array[e - 1] += 1;
    }
    if array.iter().fold(1, |e, m| e * m) != 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
