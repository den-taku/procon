use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
        ls: [i32; l],
        rs: [i32; r]
    }
    let mut count = 0;
    let mut matched = vec![false; r];
    for l in ls {
        for (i, &r) in rs.iter().enumerate() {
            if l == r && !matched[i] {
                matched[i] = true;
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
