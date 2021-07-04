use proconio::{fastout, input};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }
    let mut ans = 0;
    let mut flag = vec![0; n];
    'a: for i in (0..n).rev() {
        'b: for j in (0..n).rev() {
            if i == j {
                continue;
            }
            for k in (1..n + 1).rev() {
                if i <= flag[i] && j <= flag[i] {
                    break;
                }
                let graph = convert(&abc, k, n, i, j);
                let shortest = shortest_path(&graph, i, j);
                // println!("{} {} {} {:?}", i, j, k, shortest);
                if let Some(e) = shortest {
                    ans += e;
                } else {
                    if i + 1 <= k && j + 1 <= k {
                        flag[i] = k;
                        break 'a;
                    }
                    if j + 1 <= k {
                        break 'b;
                    }
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}

// fn convert2 (
//     abc: &[(usize, usize, usize)],
//     k: usize,
//     n: usize,
//     s: usize,
//     t: usize,
// )

fn convert(
    abc: &[(usize, usize, usize)],
    k: usize,
    n: usize,
    s: usize,
    t: usize,
) -> Vec<Vec<Edge>> {
    let mut v = vec![Vec::with_capacity(n); n];
    for &(a, b, c) in abc {
        if a <= k && b <= k
            || a == s + 1 && (b <= k || b == t + 1)
            || (a <= k || a == s + 1) && b == t + 1
        {
            unsafe {
                v.get_unchecked_mut(a - 1).push(Edge {
                    node: b - 1,
                    cost: c,
                });
            }
        }
    }
    // if s == 1 && t == 2 && k == 1 {
    //     println!("{:?}", &v);
    // }
    v
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
#[derive(Clone, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| std::usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    unsafe {
        *dist.get_unchecked_mut(start) = 0;
    }
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        unsafe {
            if cost > *dist.get_unchecked(position) {
                continue;
            }
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < unsafe { *dist.get_unchecked(next.position) } {
                heap.push(next);
                // Relaxation, we have now found a better way
                unsafe {
                    *dist.get_unchecked_mut(next.position) = next.cost;
                }
            }
        }
    }

    // Goal not reachable
    None
}
