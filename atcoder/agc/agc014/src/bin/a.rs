use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        mut c: u64
    }
    if a == b && b == c {
        println!("{}", if a % 2 == 0 { "-1" } else { "0" });
        return;
    }
    let mut v_a = Vec::new();
    v_a.push(a % 2 == 0);
    while a != 0 {
        v_a.push((a / 2) % 2 == 0);
        a /= 2;
    }
    let mut v_b = Vec::new();
    v_b.push(b % 2 == 0);
    while b != 0 {
        v_b.push((b / 2) % 2 == 0);
        b /= 2;
    }
    let mut v_c = Vec::new();
    v_c.push(c % 2 == 0);
    while c != 0 {
        v_c.push((c / 2) % 2 == 0);
        c /= 2;
    }
    for i in 0..std::cmp::max(std::cmp::max(v_a.len(), v_b.len()), v_c.len()) {
        if !(v_a[i % v_a.len()] == v_b[i % v_b.len()] && v_b[i % v_b.len()] == v_c[i % v_c.len()]) {
            println!("{}", i);
            return;
        }
    }
    println!("-1")
}
