use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }
    let mut ans = 0usize;
    let graph = convert(&abc, n);
    let mut past = graph.clone();
    let mut next = graph;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                unsafe {
                    let a = *next.get_unchecked(i * n + j);
                    *next.get_unchecked_mut(i * n + j) = min(
                        a,
                        *past.get_unchecked(i * n + k),
                        *past.get_unchecked(k * n + j),
                    );
                }
            }
        }
        ans += next
            .iter()
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .sum::<usize>() as usize;
        past = next.clone();
    }
    println!("{}", ans);
}

#[inline]
fn min(a: Option<usize>, b: Option<usize>, c: Option<usize>) -> Option<usize> {
    if let Some(e1) = a {
        if let Some(e2) = b {
            if let Some(e3) = c {
                Some(std::cmp::min(e1, e2 + e3))
            } else {
                Some(e1)
            }
        } else {
            Some(e1)
        }
    } else if let Some(e2) = b {
        if let Some(e3) = c {
            Some(e2 + e3)
        } else {
            None
        }
    } else {
        None
    }
}

#[inline]
fn convert(abc: &[(usize, usize, usize)], n: usize) -> Vec<Option<usize>> {
    let mut v = vec![None; n * n];
    for &(a, b, c) in abc {
        v[(a - 1) * n + (b - 1)] = Some(c)
    }
    for i in 0..n {
        v[i * n + i] = Some(0);
    }
    v
}
