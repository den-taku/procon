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
        for i in 0..n {
            if load[i].0 > 1000 {
                pattern = 3;
                break;
            } else if load[i].1 > 1000 {
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
}

// n < 30 || vi < 1000 && wi < 1000
fn if_n_30(n: usize, w: i64, load: Vec<(i64, i64)>) -> i64 {
    let mut a = vec![(0i64, 0i64); 2i64.pow(n as u32 / 2 as u32) as usize];
    for i in 0..a.len() {
        a[i] = adder(n, i, &load);
    }
    let mut b = vec![(0i64, 0i64); 2i64.pow(n as u32 - n as u32 / 2 as u32) us usize];
    println!("1");
    unimplemented!()
}

fn adder(n: usize, index: usize, load: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut bit = index;
    let mut value = 0;
    let mut weight = 0; 
    for i in 0..n/2 {
        if bit & 1 == 1 {
            value += load[i].0;
            weight += load[i].1;
        }
        bit = bit >> 1;
    }
    (value, weight)
}

fn if_v_over_1000(n: usize, w: i64, load: Vec<(i64, i64)>) -> i64 {
    println!("2");
    unimplemented!()
}

fn if_w_over_1000(n: usize, w: i64, load: Vec<(i64, i64)>) -> i64 {
    println!("3");
    unimplemented!()
}
