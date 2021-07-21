use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xs: [u32; n]
    }
    let (lm, um) = {
        let mut sorted = xs.clone();
        sorted.sort();
        (sorted[n / 2 - 1], sorted[n / 2])
    };
    for x in xs {
        if x <= lm {
            println!("{}", um);
        } else {
            println!("{}", lm);
        }
    }
}
