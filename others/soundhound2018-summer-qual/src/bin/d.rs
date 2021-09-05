#![allow(unreachable_code)]
use proconio::{fastout, input};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        trains: [(usize, usize, u64, u64); m]
    }
    let mut graph_yen = vec![Vec::new(); n];
    let mut graph_dollar = vec![Vec::new(); n];
    for (a, b, y, d) in trains {
        graph_yen[a - 1].push(Edge {
            node: b - 1,
            cost: y,
        });
        graph_yen[b - 1].push(Edge {
            node: a - 1,
            cost: y,
        });
        graph_dollar[a - 1].push(Edge {
            node: b - 1,
            cost: d,
        });
        graph_dollar[b - 1].push(Edge {
            node: a - 1,
            cost: d,
        });
    }
    let shortest_yen = shortest_path(&graph_yen, s - 1, 0);
    let shortest_doller = shortest_path(&graph_dollar, t - 1, 0);
    let mut queue = std::collections::BinaryHeap::new();
    for i in 0..n {
        queue.push((Reverse(shortest_yen[i] + shortest_doller[i]), i));
    }
    let mut ans = Vec::new();
    for i in 0..n {
        let mut min = std::u64::MAX;
        while let Some(&(Reverse(cand), j)) = queue.peek() {
            if j < i {
                queue.pop();
            } else {
                min = cand;
                break;
            }
        }
        ans.push((1_000_000_000_000_000 - min).to_string());
    }
    println!("{}", ans.join("\n"));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
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
#[derive(Copy, Clone)]
struct Edge {
    node: usize,
    cost: u64,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, _goal: usize) -> Vec<u64> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| std::u64::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        // if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    // None
    dist
}
