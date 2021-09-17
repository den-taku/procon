// Intersection of Two Prisms (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1313)
#![allow(unreachable_code)]

fn main() {
    while let Some((xy, xz)) = input() {
        let min1 = xy.iter().map(|e| e.0).fold(std::f64::MAX, min);
        let max1 = xy.iter().map(|e| e.0).fold(std::f64::MIN, max);
        let min2 = xz.iter().map(|e| e.0).fold(std::f64::MAX, min);
        let max2 = xz.iter().map(|e| e.0).fold(std::f64::MIN, max);
        let mut xs = xy
            .iter()
            .map(|&e| e.0)
            .chain(xz.iter().map(|&e| e.0))
            .collect::<Vec<_>>();
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut res = 0.0;
        for i in 0..xs.len() - 1 {
            let a = xs[i];
            let b = xs[i + 1];
            let c = (a + b) / 2.0;
            if min1 <= c && c <= max1 && min2 <= c && c <= max2 {
                let fa = width(&xy, a) * width(&xz, a);
                let fb = width(&xy, b) * width(&xz, b);
                let fc = width(&xy, c) * width(&xz, c);
                res += (b - a) / 6.0 * (fa + 4.0 * fc + fb);
            }
        }
        println!("{}", res);
    }
}

fn width(points: &[(f64, f64)], x: f64) -> f64 {
    let mut lb = std::f64::MAX;
    let mut ub = std::f64::MIN;
    for i in 0..points.len() {
        let x1 = points[i].0;
        let y1 = points[i].1;
        let x2 = points[(i + 1) % points.len()].0;
        let y2 = points[(i + 1) % points.len()].1;
        if (x1 - x) * (x2 - x) <= 0.0 && x1 != x2 {
            let y = y1 + (y2 - y1) * (x - x1) / (x2 - x1);
            lb = min(lb, y);
            ub = max(ub, y);
        }
    }
    max(0.0, ub - lb)
}

#[inline]
fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn min(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

#[inline]
fn input() -> Option<(Vec<(f64, f64)>, Vec<(f64, f64)>)> {
    let (m, n) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    if m == 0 && n == 0 {
        None
    } else {
        let mut xy = Vec::new();
        for _ in 0..m {
            let e = read_line::<f64>();
            xy.push((e[0], e[1]))
        }
        let mut xz = Vec::new();
        for _ in 0..n {
            let e = read_line::<f64>();
            xz.push((e[0], e[1]))
        }
        Some((xy, xz))
    }
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
