#![allow(unreachable_code)]
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: isize,
        w: isize,
        matrix: [String; h]
    }
    let mut visited = vec![false; (h * w) as usize];
    let load = convert(&matrix);
    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    queue1.push_back((0, 0));
    let mut ans = 0;
    let neiberh = [-1, 0, 1, 0];
    let neiberw = [0, 1, 0, -1];
    let jumph = [-2, -2, -2, -1, -1, 0, 1, 1, 2, 2, 2, 1, 1, 0, -1, -1];
    let jumpw = [-1, 0, 1, 1, 2, 2, 2, 1, 1, 0, -1, -1, -2, -2, -2, -1];
    'out: loop {
        while let Some((a, b)) = queue1.pop_front() {
            if visited[(a * w + b) as usize] {
                continue;
            }
            if (a, b) == (h - 1, w - 1) {
                break 'out;
            }
            visited[(a * w + b) as usize] = true;
            for (i, j) in neiberh.iter().zip(&neiberw) {
                let newh = a + i;
                let neww = b + j;
                if newh >= 0
                    && newh < h
                    && neww >= 0
                    && neww < w
                    && !visited[(newh * w + neww) as usize]
                {
                    if load[(newh * w + neww) as usize] {
                        queue1.push_back((newh, neww));
                    } else {
                        queue2.push_back((newh, neww));
                    }
                }
            }
            for (i, j) in jumph.iter().zip(&jumpw) {
                let newh = a + i;
                let neww = b + j;
                if newh >= 0
                    && newh < h
                    && neww >= 0
                    && neww < w
                    && !visited[(newh * w + neww) as usize]
                {
                    queue2.push_back((newh, neww));
                }
            }
        }
        queue1 = queue2;
        queue2 = VecDeque::new();
        ans += 1;
    }
    println!("{}", ans);
}

#[inline]
fn convert(matrix: &[String]) -> Vec<bool> {
    let mut v = Vec::with_capacity(matrix.len() * matrix[0].len());
    for s in matrix {
        for c in s.chars() {
            match c {
                '.' => v.push(true),
                _ => v.push(false),
            }
        }
    }
    v
}
