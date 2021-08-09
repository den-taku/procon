#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        s_vec: [String; h]
    }
    let ((si, sj), (gi, gj), mut visited) = convert(s_vec);
    let mut stack = Vec::new();
    stack.push((si, sj));
    let dh = [-1, 0, 1, 0];
    let dw = [0, 1, 0, -1];
    while let Some((i, j)) = stack.pop() {
        if gi == i && gj == j {
            println!("Yes");
            return;
        }
        for (dh, dw) in dh.iter().zip(dw.iter()) {
            let newh = i as isize + dh;
            let neww = j as isize + dw;
            if newh >= 0
                && newh < h
                && neww >= 0
                && neww < w
                && !visited[(newh * w + neww) as usize]
            {
                stack.push((newh as usize, neww as usize));
                visited[(newh * w + neww) as usize] = true;
            }
        }
    }
    println!("No");
}

#[inline]
fn convert(s_vec: Vec<String>) -> ((usize, usize), (usize, usize), Vec<bool>) {
    let mut v = Vec::with_capacity(s_vec.len());
    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    for (i, s) in s_vec.iter().cloned().enumerate() {
        for (j, c) in s.chars().enumerate() {
            match c {
                's' => {
                    v.push(true);
                    si = i;
                    sj = j;
                }
                'g' => {
                    v.push(false);
                    gi = i;
                    gj = j;
                }
                '.' => v.push(false),
                _ => v.push(true),
            }
        }
    }
    ((si, sj), (gi, gj), v)
}
