use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u32; n]
    }
    let mut ans = 0;
    let mut set = std::collections::HashSet::new();
    for &e in &p {
        if ans >= e {
            set.insert(e);
            while let Some(&_) = set.get(&ans) {
                ans += 1;
            }
            println!("{}", ans);
        } else {
            set.insert(e);
            println!("{}", ans);
        }
    }
}
