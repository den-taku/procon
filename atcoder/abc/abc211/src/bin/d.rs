use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut loads: [(usize, usize); m]
    }
    let mut edges = vec![Vec::new(); n];
    for &(a, b) in &loads {
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }
    let mut dis = vec![std::u64::MAX; n];
    let mut cnt = vec![0; n];
    let mut visited_from = vec![std::collections::HashSet::new(); n];
    dis[0] = 0;
    cnt[0] = 1;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    while let Some(node) = queue.pop_front() {
        for &dir in &edges[node] {
            if dis[dir] == dis[node] + 1 && !visited_from[dir].contains(&node) {
                cnt[dir] = (cnt[dir] + cnt[node]) % MOD;
                visited_from[dir].insert(node);
                if dir != n - 1 {
                    queue.push_back(dir);
                }
            } else if dis[dir] > dis[node] + 1 {
                cnt[dir] = cnt[node];
                dis[dir] = dis[node] + 1;
                visited_from[dir] = [node]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<_>>();
                if dir != n - 1 {
                    queue.push_back(dir);
                }
            }
        }
    }
    println!("{}", cnt[n - 1]);
}
