#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [u8; 4 * 4]
    }
    let mut ans = 0;
    for t in 0..1 << 16 {
        let b = 0b1111_1010_1110_1111;
        if !use_all(t, &a) {
            if t == b {
                println!("1")
            }
            continue;
        } else if !connected(t) {
            if t == b {
                println!("2")
            }
            continue;
        } else if has_pond(t) {
            if t == b {
                println!("3")
            }
            continue;
        }
        if t == b {
            println!("ok")
        }
        ans += 1;
    }
    println!("{}", ans);
}

fn has_pond(t: usize) -> bool {
    let mut covered = [false; 36];
    for i in 0..16 {
        if t >> i & 1 == 1 {
            covered[(i / 4 + 1) * 6 + (i % 4) + 1] = true;
        }
    }
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((0, 0));
    covered[0] = true;
    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    while let Some((i, j)) = queue.pop_front() {
        for (x, y) in dx.iter().zip(&dy) {
            if i + x >= 0 && i + x < 6 && j + y >= 0 && j + y < 6 {
                if !covered[((i + x) * 6 + j + y) as usize] {
                    covered[((i + x) * 6 + j + y) as usize] = true;
                    queue.push_back((i + x, j + y))
                }
            }
        }
    }
    !covered.iter().all(|&e| e)
}

fn connected(t: usize) -> bool {
    let mut visited = [false; 16];
    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    let mut queue = std::collections::VecDeque::new();
    'out: for i in 0..4 {
        for j in 0..4 {
            if t >> (i * 4 + j) & 1 == 1 {
                queue.push_back((i, j));
                visited[(i * 4 + j) as usize] = true;
                break 'out;
            }
        }
    }
    while let Some((i, j)) = queue.pop_front() {
        for (x, y) in dx.iter().zip(&dy) {
            if i + x >= 0 && i + x < 4 && j + y >= 0 && j + y < 4 {
                if !visited[((i + x) * 4 + j + y) as usize]
                    && t >> ((i + x) * 4 + j + y) & 1usize == 1usize
                {
                    visited[((i + x) * 4 + j + y) as usize] = true;
                    queue.push_back((i + x, j + y))
                }
            }
        }
    }
    for i in 0..16 {
        if (!visited[i]) && t >> i == 1 {
            return false;
        }
    }
    true
}

fn use_all(t: usize, a: &[u8]) -> bool {
    for i in 0..16 {
        if a[i] == 1 && t >> i & 1 == 0 {
            return false;
        }
    }
    true
}
