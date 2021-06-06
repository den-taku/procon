use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        input: [(usize, usize); m]
    }
    let mut loads = vec![Vec::new(); 2000];
    for &(a, b) in &input {
        loads[a - 1].push(b - 1);
    }
    let mut ans = 0;
    for i in 0..n {
        let mut flag = vec![false; n];
        dfs(i, &loads, &mut flag);
        for j in 0..n {
            if flag[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn dfs(pos: usize, loads: &Vec<Vec<usize>>, flag: &mut Vec<bool>) {
    if flag[pos] {
        return;
    } else {
        flag[pos] = true;
        for &e in &loads[pos] {
            dfs(e, loads, flag);
        }
    }
}
