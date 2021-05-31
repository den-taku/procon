use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: i64,
        load: [(i64, i64);n]
    }
    let mut pattern = 0;
    if n <= 30 {
        pattern = 1;
    } else {
        for (value, weight) in load.iter().take(n) {
            if weight > &1000 {
                pattern = 3;
                break;
            } else if value > &1000 {
                pattern = 2;
                break;
            }
        }
    }
    println!(
        "{}",
        match pattern {
            2 => if_v_over_1000(n, w, load),
            3 => if_w_over_1000(n, w, load),
            _ => if_n_30(n, w, load),
        }
    )
    // println!("{}", if_v_over_1000(n, w, load));
}

// n < 30 || vi < 1000 && wi < 1000
fn if_n_30(n: usize, w: i64, load: Vec<(i64, i64)>) -> i64 {
    let mut a = vec![(0i64, 0i64); 2i64.pow(n as u32 / 2 as u32) as usize];
    for (i, e) in a.iter_mut().enumerate() {
        *e = adder(0, n / 2, i, &load);
    }
    let a = a;
    let mut tmp = vec![(0i64, 0i64); 2i64.pow(n as u32 - n as u32 / 2 as u32) as usize];
    for (i, e) in tmp.iter_mut().enumerate() {
        *e = adder(n / 2, n, i, &load);
    }
    tmp.sort_by_key(|&(v, w)| (w, v));
    let mut b = Vec::with_capacity(2i64.pow(n as u32 - n as u32 / 2 as u32) as usize);
    if !tmp.is_empty() {
        let mut max = 0;
        for (value, weight) in &tmp {
            if weight > &w {
                break;
            }
            if value >= &max {
                b.push((*value, *weight));
            }
            max = std::cmp::max(max, *value);
        }
    }
    let b = b;
    let mut ans = (0i64, 0i64);
    for e in a {
        if e.1 > w {
            continue;
        }
        let est;
        if let Some(index) = binary_search(w - e.1, 0, b.len() - 1, &b) {
            est = (e.0 + b[index].0, e.1 + b[index].1);
        } else {
            est = (e.0, e.1);
        }
        if est.0 > ans.0 {
            ans = est;
        }
    }
    ans.0
}

fn binary_search(
    w_value: i64,
    lower_bound: usize,
    upper_bound: usize,
    b: &[(i64, i64)],
) -> Option<usize> {
    if upper_bound - lower_bound == 1 {
        if condition(b[upper_bound].1, w_value) {
            return Some(upper_bound);
        } else if condition(b[lower_bound].1, w_value) {
            return Some(lower_bound);
        } else {
            return None;
        }
    }
    let est = (lower_bound + upper_bound) / 2;
    if condition(b[est].1, w_value) {
        binary_search(w_value, est, upper_bound, b)
    } else {
        binary_search(w_value, lower_bound, est, b)
    }
}

fn condition(est: i64, w_value: i64) -> bool {
    est <= w_value
}

// 全ての組み合わせについて v と w の総和を計算する
fn adder(start: usize, end: usize, index: usize, load: &[(i64, i64)]) -> (i64, i64) {
    let mut bit = index;
    let mut value = 0;
    let mut weight = 0;
    // for i in start..end {
    for (v, w) in load.iter().take(end).skip(start) {
        if bit & 1 == 1 {
            // value += load[i].0;
            value += v;
            // weight += load[i].1;
            weight += w;
        }
        bit >>= 1;
    }
    (value, weight)
}

fn if_v_over_1000(n: usize, w: i64, load: Vec<(i64, i64)>) -> i64 {
    let mut dp = vec![0i64; (n + 1) * (w + 1) as usize];
    for i in 1..n + 1 {
        for j in 1..w as usize + 1 {
            dp[i * (w as usize + 1) + j] = if load[i - 1].1 > j as i64 {
                dp[(i - 1) * (w as usize + 1) + j]
            } else {
                std::cmp::max(
                    dp[(i - 1) * (w as usize + 1) + j],
                    dp[(i - 1) * (w as usize + 1) + j - load[i - 1].1 as usize] + load[i - 1].0,
                )
            };
        }
    }
    dp.pop().unwrap()
}

fn if_w_over_1000(n: usize, w: i64, load: Vec<(i64, i64)>) -> i64 {
    println!("3");
    println!("{:?} {:?} {:?}", n, w, load);
    unimplemented!()
}
