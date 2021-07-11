use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        queries: [(usize, usize); q]
    }
    let mut vec = vec![vec![]; n];
    for &(a, b) in &ab {
        vec[a - 1].push(b);
        vec[b - 1].push(a);
    }
    let dis = bfs(n, &vec);
    for &(c, d) in &queries {
        println!(
            "{}",
            if (dis[c - 1] + dis[d - 1]) % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        )
    }
}

#[inline]
fn bfs(n: usize, graph: &[Vec<usize>]) -> Vec<i32> {
    let mut que = std::collections::VecDeque::<usize>::with_capacity(n);
    let mut v = vec![0; n];
    que.push_back(1);
    while !que.is_empty() {
        let next = que.pop_front().unwrap();
        for &e in &graph[next - 1] {
            if v[e - 1] == 0 {
                v[e - 1] = v[next - 1] + 1;
                que.push_back(e);
            }
        }
    }
    v
}
