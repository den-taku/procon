use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
        mut c: [i32; n]
    }
    a.sort();
    b.sort();
    let mut est_a = n - 1;
    let mut buffer_tmp = vec![None; n];
    for (i, &b) in b.iter().enumerate().rev() {
        if let Some(index_a) = binary_search(b, est_a, 0, &a) {
            est_a = index_a;
            unsafe { *buffer_tmp.get_unchecked_mut(i) = Some(index_a) };
        } else {
            break;
        }
    }
    let mut buffer_b = vec![None; n];
    unsafe {
        *buffer_b.get_unchecked_mut(0) = {
            if let Some(index) = *buffer_tmp.get_unchecked(0) {
                Some(index + 1)
            } else {
                None
            }
        }
    };
    for i in 1..n {
        if let Some(index) = unsafe { buffer_tmp.get_unchecked(i) } {
            if let Some(sum) = unsafe { buffer_b.get_unchecked(i - 1) } {
                unsafe {
                    *buffer_b.get_unchecked_mut(i) = Some(sum + index + 1);
                }
            } else {
                unsafe {
                    *buffer_b.get_unchecked_mut(i) = Some(index + 1);
                }
            }
        }
    }

    let mut ans = 0i64;
    c.sort();
    let mut est_b = n - 1;
    for &c in c.iter().rev() {
        if let Some(index_b) = binary_search(c, est_b, 0, &b) {
            est_b = index_b;
            if let Some(sum) = unsafe { *buffer_b.get_unchecked(index_b) } {
                ans += sum as i64;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
}

#[inline]
fn binary_search(
    value: i32,
    mut upper_bound: usize,
    mut lower_bound: usize,
    slice: &[i32],
) -> Option<usize> {
    let mut est = (upper_bound + lower_bound) / 2;
    while upper_bound - lower_bound >= 2 {
        if condition(est, value, slice) {
            lower_bound = est;
        } else {
            upper_bound = est;
        }
        est = (upper_bound + lower_bound) / 2;
    }
    if condition(upper_bound, value, slice) {
        Some(upper_bound)
    } else if condition(lower_bound, value, slice) {
        Some(lower_bound)
    } else {
        None
    }
}

#[inline]
fn condition(est: usize, value: i32, slice: &[i32]) -> bool {
    unsafe { *slice.get_unchecked(est) < value }
}
