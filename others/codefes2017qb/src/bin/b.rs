use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        dificulties: [u32; n],
        m: usize,
        needed: [u32; m]
    }
    if n < m {
        println!("NO")
    } else {
        let mut f = std::collections::HashMap::new();
        for d in dificulties {
            if let Some(&e) = f.get(&d) {
                f.insert(d, e + 1);
            } else {
                f.insert(d, 1);
            }
        }
        for t in needed {
            if let Some(&e) = f.get(&t) {
                if e == 1 {
                    f.remove(&t);
                } else {
                    f.insert(t, e - 1);
                }
            } else {
                println!("NO");
                return;
            }
        }
        println!("YES");
    }
}
