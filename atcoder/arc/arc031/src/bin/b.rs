#![allow(unreachable_code)]
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        map: [Bytes; 10]
    }
    let mut visited = map
        .into_iter()
        .flatten()
        .map(|e| e == b'x')
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut neiber = visited
        .iter()
        .map(|&e| if e { Some(0) } else { None })
        .collect::<Vec<_>>();
    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    while let Some(index) = visited.iter().position(|e| !e) {
        count += 1;
        visited[index] = true;
        let mut marked = [false; 10 * 10];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(index);
        while let Some(i) = queue.pop_front() {
            for j in 0..4 {
                let newx = (i / 10) as isize + dx[j];
                let newy = (i % 10) as isize + dy[j];
                if newx >= 0 && newx < 10 && newy >= 0 && newy < 10 {
                    if !visited[(newx * 10 + newy) as usize] {
                        visited[(newx * 10 + newy) as usize] = true;
                        queue.push_back((newx * 10 + newy) as usize);
                    } else if !marked[(newx * 10 + newy) as usize] {
                        marked[(newx * 10 + newy) as usize] = true;
                        if let Some(v) = neiber[(newx * 10 + newy) as usize] {
                            neiber[(newx * 10 + newy) as usize] = Some(v + 1)
                        }
                    }
                }
            }
        }
    }
    println!(
        "{}",
        if count == 1
            || neiber
                .iter()
                .any(|&e| if let Some(v) = e { v >= count } else { false })
        {
            "YES"
        } else {
            "NO"
        }
    );
}
