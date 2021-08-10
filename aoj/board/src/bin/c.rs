// How Many Islands?
// (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1160&lang=jp)
#![allow(unreachable_code)]

fn main() {
    while let Some((w, h, mut visited)) = input() {
        if w == 0 && h == 0 {
            break;
        }
        let mut count = 0;
        let dx = [-1, -1, 0, 1, 1, 1, 0, -1];
        let dy = [0, 1, 1, 1, 0, -1, -1, -1];
        while let Some(index) = visited.iter().position(|e| !e) {
            count += 1;
            visited[index] = true;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(index);
            while let Some(i) = queue.pop_front() {
                for j in 0..8 {
                    let newx = (i / w as usize) as isize + dx[j];
                    let newy = (i % w as usize) as isize + dy[j];
                    if newx >= 0 && newx < h && newy >= 0 && newy < w {
                        if !visited[(newx * w + newy) as usize] {
                            visited[(newx * w + newy) as usize] = true;
                            queue.push_back((newx * w + newy) as usize);
                        }
                    }
                }
            }
        }
        println!("{}", count);
    }
}

#[inline]
fn input() -> Option<(isize, isize, Vec<bool>)> {
    let (w, h) = {
        let one = read_line::<isize>();
        if one.len() == 0 {
            return None;
        }
        (one[0], one[1])
    };
    let mut v = Vec::with_capacity(h as usize);
    for _ in 0..h {
        v.push(
            read_line::<u8>()
                .iter()
                .map(|&e| e == 0)
                .collect::<Vec<_>>(),
        )
    }
    Some((w, h, v.iter().flatten().copied().collect()))
}

#[inline]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}
