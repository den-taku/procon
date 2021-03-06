#![allow(dead_code)]
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
        q: usize,
        queries: [(usize, usize, String); q]
    }
    let mut tree = segment_tree_library::SegmentTree::<u32, _>::new(s.len(), |a, b| a | b, 0);
    for (i, c) in s.chars().enumerate() {
        let alphabet = 1 << ((c as u8 - b'a') as usize);
        tree.update(i, alphabet);
    }
    let mut ans = Vec::new();
    for (q, l, r) in queries {
        if q == 1 {
            let alphabet = 1 << ((r.chars().next().unwrap() as u8 - b'a') as usize);
            tree.update(l - 1, alphabet);
        } else {
            let r = r.parse().unwrap();
            let mut bit = tree.find(l - 1..r);
            let mut count = 0;
            for _ in 0..26 {
                if bit & 1 == 1 {
                    count += 1;
                }
                bit >>= 1
            }
            ans.push(count.to_string());
        }
    }
    println!("{}", ans.join("\n"))
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
