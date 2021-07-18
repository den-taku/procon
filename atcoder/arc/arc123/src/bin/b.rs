use proconio::{fastout, input};

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        mut b: [u32; n],
        mut c: [u32; n]
    }
    a.sort();
    b.sort();
    c.sort();
    let mut count = 0;
    let mut i_a = 0;
    let mut i_b = 0;
    let mut i_c = 0;
    'a: loop {
        if i_a == n {
            break 'a;
        } else {
            let a = a[i_a];
            'b: loop {
                if i_b == n {
                    break 'a;
                } else {
                    let b = b[i_b];
                    if a < b {
                        loop {
                            if i_c == n {
                                break 'a;
                            } else {
                                let c = c[i_c];
                                if b < c {
                                    count += 1;
                                    i_a += 1;
                                    i_b += 1;
                                    i_c += 1;
                                    break 'b;
                                } else {
                                    i_c += 1;
                                }
                            }
                        }
                    } else {
                        i_b += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
