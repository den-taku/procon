#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, u64); n - 1],
        d: [u64; n]
    }
    let mut adjacent = vec![vec![]; n];
    let mut cost = vec![vec![]; n];
    for (u, v, w) in edges {
        adjacent[u - 1].push(v - 1);
        cost[u - 1].push(w);
        adjacent[v - 1].push(u - 1);
        cost[v - 1].push(w);
    }
    let mut nodes = vec![];
    let f = |a, b| std::cmp::max(a, b);
    for v in &adjacent {
        let tree = segment_tree_library::SegmentTree::new(v.len(), f, 0);
        nodes.push(tree);
    }
    let mut parents: Vec<Option<usize>> = vec![None; n];
    for (i, &_sub) in adjacent[0].iter().enumerate() {
        let value = dfs(0, i, &adjacent, &cost, &d, &mut parents, &mut nodes);
        // let sub_ans = std::cmp::max(d[0], value + cost[0][i]);
        let sub_ans = value;
        nodes[0].update(i, sub_ans);
    }
    let mut dp = d.clone();
    // dp[0] = std::cmp::max(nodes[0].find(0..adjacent[0].len()), d[0]);
    dp[0] = nodes[0].find(0..adjacent[0].len());
    for (i, &_sub) in adjacent[0].iter().enumerate() {
        dfs2(0, i, &adjacent, &cost, &d, &parents, &mut nodes, &mut dp);
    }
    for value in dp {
        println!("{}", value);
    }
}

fn dfs2<F>(
    parent: usize,
    i: usize,
    adjacent: &[Vec<usize>],
    cost: &[Vec<u64>],
    d: &[u64],
    parents: &[Option<usize>],
    nodes: &mut [segment_tree_library::SegmentTree<u64, F>],
    dp: &mut [u64],
) where
    F: Fn(u64, u64) -> u64,
{
    let node = adjacent[parent][i];
    let left = nodes[parent].find(0..i);
    let right = nodes[parent].find(i + 1..adjacent[parent].len());
    let value = std::cmp::max(left, right);
    let parent_ans = std::cmp::max(d[parent], value) + cost[parent][i];
    let index = parents[node].unwrap();
    nodes[node].update(index, parent_ans);
    dp[node] = nodes[node].find(0..adjacent[node].len());
    for (j, &sub) in adjacent[node].iter().enumerate() {
        if sub == parent {
            continue;
        }
        dfs2(node, j, adjacent, cost, d, parents, nodes, dp);
    }
}

fn dfs<F>(
    parent: usize,
    i: usize,
    adjacent: &[Vec<usize>],
    cost: &[Vec<u64>],
    d: &[u64],
    parents: &mut [Option<usize>],
    nodes: &mut [segment_tree_library::SegmentTree<u64, F>],
) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    let node = adjacent[parent][i];
    for (j, &sub) in adjacent[node].iter().enumerate() {
        if sub == parent {
            parents[node] = Some(j);
            continue;
        }
        let value = dfs(node, j, adjacent, cost, d, parents, nodes);
        // let sub_ans = std::cmp::max(d[node], value + cost[node][j]);
        let sub_ans = value;
        nodes[node].update(j, sub_ans);
    }
    if adjacent[node].len() == 1 {
        // This is a leaf
        d[node] + cost[parent][i]
    } else {
        std::cmp::max(nodes[node].find(0..adjacent[node].len()), d[node]) + cost[parent][i]
    }
}

/// Segment Tree for Monoid (T, F)
/// T: Type
/// F: Binomial operator
///
/// new
/// update
/// find
pub mod segment_tree_library {
    /// verified (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5953641#1)
    /// Segment Tree for Monoid (T, F)
    #[derive(Debug)]
    pub struct SegmentTree<T, F>
    where
        F: Fn(T, T) -> T,
    {
        tree: Vec<T>,
        n: usize,
        f: F,
        unit: T,
    }

    impl<T, F> SegmentTree<T, F>
    where
        T: Copy,
        F: Fn(T, T) -> T,
    {
        /// O(n)
        #[inline]
        pub fn new(n: usize, f: F, unit: T) -> Self {
            let mut n_ = 1;
            while n_ < n {
                n_ *= 2
            }
            Self {
                tree: vec![unit; n_ * 2 - 1],
                n: n_,
                f,
                unit,
            }
        }

        /// O(lg n)
        /// a_index <- value
        #[inline]
        pub fn update(&mut self, index: usize, value: T) {
            if index >= self.n {
                panic!("SegmentTree: out of bound")
            }
            let mut k = self.n - 1 + index;
            self[k] = value;
            while k > 0 {
                k = (k - 1) / 2;
                self[k] = (self.f)(self[k * 2 + 1], self[k * 2 + 2])
            }
        }

        /// O(lg n)
        /// return a_s * a_s+1 * ... * a_t
        #[inline]
        pub fn find(&self, range: std::ops::Range<usize>) -> T {
            if range.end <= range.start {
                self.unit
            } else {
                self.find_rec(range, 0, 0, self.n)
            }
        }

        fn find_rec(&self, range: std::ops::Range<usize>, k: usize, l: usize, r: usize) -> T {
            if r <= range.start || range.end <= l {
                self.unit
            } else if range.start <= l && r <= range.end {
                self[k]
            } else {
                (self.f)(
                    self.find_rec(range.clone(), k * 2 + 1, l, (l + r) / 2),
                    self.find_rec(range, k * 2 + 2, (l + r) / 2, r),
                )
            }
        }
    }

    impl<T, F> std::ops::Index<usize> for SegmentTree<T, F>
    where
        F: Fn(T, T) -> T,
    {
        type Output = T;
        #[inline]
        fn index(&self, index: usize) -> &Self::Output {
            &self.tree[index]
        }
    }

    impl<T, F> std::ops::IndexMut<usize> for SegmentTree<T, F>
    where
        F: Fn(T, T) -> T,
    {
        #[inline]
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.tree[index]
        }
    }

    #[cfg(test)]
    mod tests_segment_tree {
        use super::*;

        #[test]
        fn for_rmq1() {
            let n = 3;
            let _q = 5;
            let com = [(0, 0, 1), (0, 1, 2), (0, 2, 3), (1, 0, 2), (1, 1, 2)];
            let answers = &[1, 2];
            let mut rmq = SegmentTree::new(n, std::cmp::min, std::usize::MAX);
            let mut i = 0;
            for &(d, x, y) in &com {
                match d {
                    0 => rmq.update(x, y),
                    1 => {
                        assert_eq!(rmq.find(x..y + 1), answers[i]);
                        i += 1;
                    }
                    _ => unreachable!(),
                }
            }
            let n = 1;
            let _q = 3;
            let com = [(1, 0, 0), (0, 0, 5), (1, 0, 0)];
            let answers = &[std::usize::MAX, 5];
            let mut rmq = SegmentTree::new(n, std::cmp::min, std::usize::MAX);
            let mut i = 0;
            for &(d, x, y) in &com {
                match d {
                    0 => rmq.update(x, y),
                    1 => {
                        assert_eq!(rmq.find(x..y + 1), answers[i]);
                        i += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
