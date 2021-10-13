use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        d: usize,
        k: usize,
        lr: [(usize, usize); d],
        mut st: [(usize, usize); k]
    }
    let mut ans = Vec::with_capacity(k);
    for (s, t) in st.iter_mut() {
        for (time, &(l, r)) in lr.iter().enumerate() {
            if *s < l || r < *s {
                continue;
            }
            if s < t {
                *s = std::cmp::max(*s, r);
                if *t <= *s {
                    ans.push((time + 1).to_string());
                    break;
                }
            } else {
                *s = std::cmp::min(*s, l);
                if *s <= *t {
                    ans.push((time + 1).to_string());
                    break;
                }
            }
        }
    }
    println!("{}", ans.join("\n"))
}
