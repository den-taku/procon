use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        dogs: [(i64, char); 2*n]
    }
    let mut red = Vec::with_capacity(2 * n);
    let mut blu = Vec::with_capacity(2 * n);
    let mut grn = Vec::with_capacity(2 * n);
    let mut c_red = 0;
    let mut c_ble = 0;
    let mut c_grn = 0;
    for d in dogs {
        match d.1 {
            'R' => {
                c_red += 1;
                red.push(d.0);
            }
            'G' => {
                c_ble += 1;
                blu.push(d.0);
            }
            'B' => {
                c_grn += 1;
                grn.push(d.0);
            }
            _ => unreachable!(),
        }
    }
    if c_grn % 2 == 0 {
        if c_ble % 2 == 0 {
            if c_red % 2 == 0 {
                println!("0");
            } else {
                unreachable!();
            }
        } else {
            if c_red % 2 == 0 {
                unreachable!();
            } else {
                // blue & red == odd
                get_min(blu, red, grn);
            }
        }
    } else {
        if c_ble % 2 == 0 {
            if c_red % 2 == 0 {
                unreachable!()
            } else {
                // green & red == odd
                get_min(grn, red, blu);
            }
        } else {
            if c_red % 2 == 0 {
                // green & blue == odd
                get_min(grn, blu, red)
            } else {
                unreachable!()
            }
        }
    }
}

fn get_min(one: Vec<i64>, mut two: Vec<i64>, mut three: Vec<i64>) {
    two.sort();
    three.sort();
    let mut two_color = std::i64::MAX;
    let mut three_color_first = std::i64::MAX;
    let mut three_color_second = std::i64::MAX;
    let ans;
    for e in one {
        // for tow
        let est;
        let index = binary_serch(e, 0, two.len() - 1, &two);
        if index == two.len() - 1 {
            est = (e - two[index]).abs();
        } else {
            let s = (e - two[index]).abs();
            let l = (e - two[index + 1]).abs();
            est = if s < l { s } else { l };
        }
        if two_color > est {
            two_color = est;
        }
        // for three
        if three.len() != 0 {
            let est;
            let index = binary_serch(e, 0, three.len() - 1, &three);
            if index == three.len() - 1 {
                est = (e - three[index]).abs();
            } else {
                let s = (e - three[index]).abs();
                let l = (e - three[index + 1]).abs();
                est = if s < l { s } else { l };
            }
            if three_color_first > est {
                three_color_first = est;
            }
        }
    }
    if three.len() != 0 {
        for e in two {
            // for three
            let est;
            let index = binary_serch(e, 0, three.len() - 1, &three);
            if index == three.len() - 1 {
                est = (e - three[index]).abs();
            } else {
                let s = (e - three[index]).abs();
                let l = (e - three[index + 1]).abs();
                est = if s < l { s } else { l };
            }
            if three_color_second > est {
                three_color_second = est;
            }
        }
    }
    let three_color = if three.len() != 0 { three_color_first + three_color_second } else { std::i64::MAX };
    ans = if two_color > three_color {
        three_color
    } else {
        two_color
    };
    println!("{}", ans);
}

fn binary_serch(value: i64, lower_bound: usize, upper_bound: usize, array: &Vec<i64>) -> usize {
    if upper_bound == lower_bound {
        return upper_bound;
    }
    if upper_bound - lower_bound == 1 {
        if condition(value, array[upper_bound]) {
            return upper_bound;
        } else {
            return lower_bound;
        }
    } else {
        let est = (lower_bound + upper_bound) / 2;
        if condition(value, array[est]) {
            return binary_serch(value, est, upper_bound, array);
        } else {
            return binary_serch(value, lower_bound, est, array);
        }
    }
}

fn condition(value: i64, est: i64) -> bool {
    value >= est
}