use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i128,
        t: i128,
        ants: [(i128, i128); n]
    }
    // (X, W) -> (sorted_X, W, past_index)
    let ants = {
        let mut ants: Vec<(i128, i128, usize)> = ants
            .iter()
            .enumerate()
            .map(|e| ((*e.1).0, (*e.1).1, e.0))
            .collect();
        ants.sort();
        ants
    };
    // times that ants[0] touch another while t
    let count = estmated_index(&ants, t, l);
    let dis = count % l;
    // virtual index
    let index = if ants[0].1 == 1 {
        (ants[0].0 + dis) % l
    } else {
        l - ((dis - ants[0].0) % l)
    };
    let mut array = vec![0i128; n];
    // i: sorted address, j: virtual address
    for (i, j) in (index..index + n as i128).enumerate() {
        array[ants[i % n].2] =
            calurate_lacation(ants[j as usize % n].0, t, ants[j as usize % n].1, l);
    }
    // println!("{}", count);
    // for i in index..index + n as i128 {
    //     array[ants[i as usize % n].2] =
    //         calurate_lacation(ants[i as usize % n].0, t, ants[i as usize % n].1, l);
    // }
    // for i in 0..ants.len() {
    //     // because 1-index and 0-index is mixed
    //     let address = array[(i + n - 1) % n];
    //     // let address = if address == 0 { l } else { address };
    //     println!("{}", address % l);
    // }
    for e in array {
        println!("{}", e % l);
    }
}

fn estmated_index(ants: &Vec<(i128, i128, usize)>, time: i128, length: i128) -> i128 {
    let mut count = 0i128;
    let base = time / length;
    let res = time % length;
    for e in ants {
        if !ants[0].1 == e.1 {
            count += base * 2;
            let distance = distance(ants[0].0, e.0, length, ants[0].1);
            if res > length / 2 + distance {
                count += 2;
            } else if res > distance {
                count += 1;
            }
            // if res > distance(ants[0].0, e.0, length, ants[0].1) {
            //     count += 1;
            // }
        }
    }
    count
}

fn calurate_lacation(first_position: i128, time: i128, direction: i128, length: i128) -> i128 {
    let est = if direction == 1 {
        (first_position + time) % length
    } else {
        length - ((time - first_position) % length)
    };
    if est == 8 {
        0
    } else {
        est
    }
}

fn distance(i: i128, j: i128, l: i128, d: i128) -> i128 {
    if d == 1 {
        if j > i {
            j - i
        } else {
            l - j + i
        }
    } else {
        if j < i {
            i - j
        } else {
            l - i + j
        }
    }
}
