use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        size: usize,
        v: [i32; size]
    }
    let mut v_k = Vec::with_capacity(size);
    v_k.push(-1i32);
    for &a in &v {
        unsafe {
            if a > *v_k.get_unchecked(v_k.len() - 1) {
                v_k.push(a);
            } else {
                // binary_search
                // condition max a[i]: a[i-1] < a
                let k_len = v_k.len();
                let mut lower_bound = 1;
                let mut upper_bound = k_len - 1;
                while upper_bound - lower_bound > 1 {
                    let index = (upper_bound + lower_bound) / 2;
                    if *v_k.get_unchecked(index - 1) < a {
                        lower_bound = index;
                    } else {
                        upper_bound = index;
                    }
                }
                let index = if *v_k.get_unchecked(upper_bound - 1) < a {
                    upper_bound
                } else {
                    lower_bound
                };
                let p = v_k.get_unchecked_mut(index);
                *p = a;
            }
        }
    }
    println!("{}", v_k.len() - 1)
}
