use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    let mut queue = std::collections::VecDeque::with_capacity(n);
    for (i, &e) in a.iter().enumerate() {
        if i % 2 == 0 {
            queue.push_back(e)
        } else {
            queue.push_front(e)
        }
    }
    if n % 2 == 0 {
        for e in queue.iter().take(n - 1) {
            print!("{} ", e);
        }
        println!("{}", queue[n - 1])
    } else {
        for &e in queue.iter().rev().take(n - 1) {
            print!("{} ", e);
        }
        println!("{}", queue[0])
    }
}
