#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};
use segment_tree_library::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, u64); n - 1],
        d: [u64; n]
    }
    let mut adjacent = vec![vec![]; n];
    let mut cost = vec![vec![]; n];
    let mut dp = vec![vec![]; n];
    for (u, v, c) in edges {
        adjacent[u - 1].push(v - 1);
        cost[u - 1].push(c);
        dp[u - 1].push(None);
        adjacent[v - 1].push(u - 1);
        cost[v - 1].push(c);
        dp[v - 1].push(None);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    for i in 0..adjacent[0].len() {
        rec(0, i, &mut dp, &adjacent, &cost, &d, &mut visited);
    }
    println!("{:?}", dp);
}

fn rec(
    start: usize,
    i: usize,
    dp: &mut [Vec<Option<u64>>],
    adjacent: &[Vec<usize>],
    cost: &[Vec<u64>],
    d: &[u64],
    visited: &mut [bool],
) -> u64 {
    let to = adjacent[start][i];
    visited[to] = true;
    if let Some(value) = dp[start][i] {
        value
    } else {
        let mut max = 0;
        for (j, &v) in adjacent[to].iter().enumerate() {
            if visited[v] {
                max = std::cmp::max(dp[to][j].unwrap(), max);
            }
            visited[v] = true;
            max = std::cmp::max(rec(to, j, dp, adjacent, cost, d, visited), max);
        }
        let ans = std::cmp::max(d[to], max) + cost[start][i];
        dp[start][i] = Some(ans);
        ans
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
    /// verified (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5939671#2)
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
            self.find_rec(range, 0, 0, self.n)
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
