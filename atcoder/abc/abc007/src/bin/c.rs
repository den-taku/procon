use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        (sy, sx): (usize, usize),
        (gy, gx): (usize, usize),
        cs: [String; r]
    }
    let (sx, sy) = (sx - 1, sy - 1);
    let (gx, gy) = (gx - 1, gy - 1);
    let cs: Vec<char> = cs
        .iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .flatten()
        .collect();
    let mut dis = vec![None; r * c];
    let mut queue = std::collections::VecDeque::with_capacity(r * c);
    let mut ans = 0;
    queue.push_back((sx, sy));
    dis[sy * c + sx] = Some(0);
    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    'out: while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for i in 0..4 {
            let distance = dis[y * c + x].unwrap();
            let x_new = x as isize + dx[i];
            let y_new = y as isize + dy[i];
            if x_new >= 0
                && x_new < c as isize
                && y_new >= 0
                && r as isize > y_new
                && dis[y_new as usize * c + x_new as usize].is_none()
                && cs[y_new as usize * c + x_new as usize] == '.'
            {
                let x_new = x_new as usize;
                let y_new = y_new as usize;
                dis[y_new * c + x_new] = Some(distance + 1);
                queue.push_back((x_new, y_new));
                if (x_new, y_new) == (gx, gy) {
                    ans = distance + 1;
                    break 'out;
                }
            }
        }
    }
    println!("{}", ans);
}
