#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [u32; h * w]
    }
    let sumh = {
        let mut v = vec![0; h];
        for (i, p) in v.iter_mut().enumerate() {
            *p = a.iter().skip(i * w).take(w).sum::<u32>();
        }
        v
    };
    let sumw = {
        let mut v = vec![0; w];
        for (j, p) in v.iter_mut().enumerate() {
            *p = a.iter().skip(j).step_by(w).sum::<u32>();
        }
        v
    };
    for (i, &si) in sumh.iter().enumerate() {
        for (j, &sj) in sumw.iter().enumerate() {
            print!("{} ", si + sj - a[i * w + j])
        }
        println!("");
    }
}
