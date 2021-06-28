use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        size: usize,
        miss_max: usize,
        v: [i32; size]
    }
    let mut v_k = vec![vec![-2i32]; miss_max + 1];
    for &a in &v {
        for k in (0..miss_max + 1).rev() {
            unsafe {
                if a > *v_k
                    .get_unchecked(k)
                    .get_unchecked(v_k.get_unchecked(k).len() - 1)
                {
                    v_k.get_unchecked_mut(k).push(a);
                } else {
                    // binary_search
                    // condition max a[i]: a[i-1] < a
                    let k_len = v_k.get_unchecked(k).len();
                    let mut lower_bound = 1;
                    let mut upper_bound = k_len - 1;
                    while upper_bound - lower_bound > 1 {
                        let index = (upper_bound + lower_bound) / 2;
                        if *v_k.get_unchecked(k).get_unchecked(index - 1) < a {
                            lower_bound = index;
                        } else {
                            upper_bound = index;
                        }
                    }
                    let index = if *v_k.get_unchecked(k).get_unchecked(upper_bound - 1) < a {
                        upper_bound
                    } else {
                        lower_bound
                    };
                    let p = v_k.get_unchecked_mut(k).get_unchecked_mut(index);
                    *p = a;
                    if k != 0 {
                        // this action break sorted condition
                        let k_1_len = v_k.get_unchecked(k - 1).len();
                        if k_1_len == k_len {
                            v_k.get_unchecked_mut(k).push(a);
                        } else {
                            let cmp_value = *v_k.get_unchecked(k).get_unchecked(k_1_len);
                            let p = v_k.get_unchecked_mut(k).get_unchecked_mut(k_1_len);
                            *p = std::cmp::min(cmp_value, a);
                        }
                    }
                }
            }
        }
    }
    println!("{}", unsafe { v_k.get_unchecked(miss_max).len() - 1 })
}
