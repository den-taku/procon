fn main() {
    loop {
        let (w, h, c) = input();
        if w == 0 && h == 0 {
            break;
        } else {
            println!("{}", count(w, h, c));
        }
    }
}

#[inline]
fn count(w: usize, h: usize, c: Vec<i32>) -> i32 {
    let mut flag = vec![false; w * h];
    let mut est = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i * w + j] == 1 && !flag[i * w + j] {
                est += 1;
                dfs(i, j, w, h, &c, &mut flag);
            }
        }
    }
    est
}

#[inline]
fn dfs(i: usize, j: usize, w: usize, h: usize, c: &[i32], flag: &mut [bool]) {
    let mut stack = Vec::with_capacity(w * h);
    stack.push((i, j));
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        flag[x * w + y] = true;
        if x > 0 && y > 0 && c[(x - 1) * w + y - 1] == 1 && !flag[(x - 1) * w + y - 1] {
            stack.push((x - 1, y - 1))
        }
        if x > 0 && c[(x - 1) * w + y] == 1 && !flag[(x - 1) * w + y] {
            stack.push((x - 1, y))
        }
        if x > 0 && y + 1 < w && c[(x - 1) * w + y + 1] == 1 && !flag[(x - 1) * w + y + 1] {
            stack.push((x - 1, y + 1))
        }
        if y > 0 && c[x * w + y - 1] == 1 && !flag[x * w + y - 1] {
            stack.push((x, y - 1))
        }
        if y + 1 < w && c[x * w + y + 1] == 1 && !flag[x * w + y + 1] {
            stack.push((x, y + 1))
        }
        if x + 1 < h && y > 0 && c[(x + 1) * w + y - 1] == 1 && !flag[(x + 1) * w + y - 1] {
            stack.push((x + 1, y - 1))
        }
        if x + 1 < h && c[(x + 1) * w + y] == 1 && !flag[(x + 1) * w + y] {
            stack.push((x + 1, y))
        }
        if x + 1 < h && y + 1 < w && c[(x + 1) * w + y + 1] == 1 && !flag[(x + 1) * w + y + 1] {
            stack.push((x + 1, y + 1))
        }
    }
}

#[inline]
fn input() -> (usize, usize, Vec<i32>) {
    let (w, h) = {
        let input = read_line::<usize>();
        (input[0], input[1])
    };
    let mut c = Vec::with_capacity(w);
    (0..h).fold((), |_, _| c.push(read_line::<i32>()));
    (w, h, c.iter().flatten().map(|e| *e).collect())
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
