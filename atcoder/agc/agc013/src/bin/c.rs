use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        t: i64,
        ant: [(i64, i64); n]
    }
    let t = t % l;
    for i in 0..n {
        let index = estmated_index(i);
        println!("{}", calurate_lacation(ant[index].0, t, ant[index].1, l));
    }
}

fn estmated_index(index: usize) -> usize {
    unimplemented!()
}

fn calurate_lacation(first_position: i64, time: i64, direction: i64, length: i64) -> i64 {
    let est = if direction == 1 {
        (first_position + time) % length
    } else {
        length - ((time - first_position) % length)
    };
    if est == 8 {
        0
    } else {
        est
    }
}
