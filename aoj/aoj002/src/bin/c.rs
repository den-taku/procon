// RSQ and RAQ (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_G&lang=jp)
#![allow(dead_code)]
#![allow(unreachable_code)]
use Query::*;

fn main() {
    let (n, queries) = input();
    let mut tree = segment_tree_delay_library::DelayedSegmentTree::new(
        n,
        |a, b| a + b,
        0,
        |a, b, c| a + b * c,
    );
    for query in queries {
        match query {
            Add(s, t, value) => tree.act_in_range(s - 1..t, value),
            GetSum(s, t) => println!("{}", tree.find(s - 1..t)),
        }
    }
}

fn input() -> (usize, Vec<Query>) {
    let (n, q) = {
        let e = read_line::<usize>();
        (e[0], e[1])
    };
    let mut v = vec![];
    for _ in 0..q {
        let e = read_line::<usize>();
        if e[0] == 0 {
            v.push(Add(e[1], e[2], e[3]))
        } else {
            v.push(GetSum(e[1], e[2]))
        }
    }
    (n, v)
}

enum Query {
    Add(usize, usize, usize),
    GetSum(usize, usize),
}

#[inline]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

pub mod segment_tree_delay_library {
    /// Delayed Segment Tree for M-act Monoid
    /// (T, F) : Monoid
    /// (U, G) : Monoid
    /// act: T × U -> T
    /// isomorphic:
    /// (xy) act m = (x act m) (y act m)
    /// x act (mn) = (x act m) (x act n)
    ///
    /// In this implementation, Act is @ and ^, then return a_s * ... a_t @ m ^ range
    /// Act: T × T × U -> T, (a_s * ... a_t) @ m ^ range, a_i, m ∈ T, range ∈ U
    /// unit of U is 0 since U is usize
    ///
    /// verified (https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5940599#1)
    #[derive(Debug)]
    pub struct DelayedSegmentTree<T, F, Act>
    where
        F: Fn(T, T) -> T,
        Act: Fn(T, T, usize) -> T,
    {
        tree: Vec<T>,
        tree_range: Vec<T>,
        n: usize,
        f: F,
        unit: T,
        act: Act,
    }

    impl<T, F, Act> DelayedSegmentTree<T, F, Act>
    where
        T: Copy,
        F: Fn(T, T) -> T,
        Act: Fn(T, T, usize) -> T,
    {
        /// O(n)
        #[inline]
        pub fn new(n: usize, f: F, unit: T, act: Act) -> Self {
            let mut n_ = 1;
            while n_ < n {
                n_ *= 2
            }
            Self {
                tree: vec![unit; n_ * 2 - 1],
                tree_range: vec![unit; n_ * 2 - 1],
                n: n_,
                f,
                unit,
                act,
            }
        }

        /// O(n)
        #[inline]
        pub fn init(n: usize, f: F, unit: T, act: Act, initializer1: T, initializer2: T) -> Self {
            let mut n_ = 1;
            while n_ < n {
                n_ *= 2
            }
            Self {
                tree: vec![initializer1; n_ * 2 - 1],
                tree_range: vec![initializer2; n_ * 2 - 1],
                n: n_,
                f,
                unit,
                act,
            }
        }

        /// O(lg n)
        /// a_index <- value
        #[inline]
        pub fn update(&mut self, index: usize, value: T) {
            if index >= self.n {
                panic!("DelayedSegmentTree: out of bound")
            }
            let mut k = self.n - 1 + index;
            self[k] = value;
            while k > 0 {
                k = (k - 1) / 2;
                self[k] = (self.f)(self[k * 2 + 1], self[k * 2 + 2])
            }
        }

        /// act in range [a, b)
        #[inline]
        pub fn act_in_range(&mut self, range: std::ops::Range<usize>, value: T) {
            if range.end > self.n {
                panic!("DelayedSegmentTree: out of bound")
            }
            self.act_in_range_rec(range, value, 0, 0, self.n)
        }

        fn act_in_range_rec(
            &mut self,
            range: std::ops::Range<usize>,
            value: T,
            k: usize,
            l: usize,
            r: usize,
        ) {
            if range.start <= l && r <= range.end {
                self.tree_range[k] = (self.act)(self.tree_range[k], value, 1)
            } else if l < range.end && range.start < r {
                self[k] = (self.act)(
                    self[k],
                    value,
                    std::cmp::min(range.end, r) - std::cmp::max(range.start, l),
                );
                self.act_in_range_rec(range.clone(), value, k * 2 + 1, l, (l + r) / 2);
                self.act_in_range_rec(range, value, k * 2 + 2, (l + r) / 2, r);
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
                (self.act)(self[k], self.tree_range[k], r - l)
            } else {
                (self.act)(
                    (self.f)(
                        self.find_rec(range.clone(), k * 2 + 1, l, (l + r) / 2),
                        self.find_rec(range.clone(), k * 2 + 2, (l + r) / 2, r),
                    ),
                    self.tree_range[k],
                    std::cmp::min(range.end, r) - std::cmp::max(range.start, l),
                )
            }
        }
    }

    impl<T, F, Act> std::ops::Index<usize> for DelayedSegmentTree<T, F, Act>
    where
        F: Fn(T, T) -> T,
        Act: Fn(T, T, usize) -> T,
    {
        type Output = T;
        #[inline]
        fn index(&self, index: usize) -> &Self::Output {
            &self.tree[index]
        }
    }

    impl<T, F, Act> std::ops::IndexMut<usize> for DelayedSegmentTree<T, F, Act>
    where
        F: Fn(T, T) -> T,
        Act: Fn(T, T, usize) -> T,
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
            let mut rmq = DelayedSegmentTree::init(
                n,
                std::cmp::min,
                std::usize::MAX,
                |a, _b, _c| a,
                std::usize::MAX,
                0,
            );
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
            let mut rmq = DelayedSegmentTree::init(
                n,
                std::cmp::min,
                std::usize::MAX,
                |a, _b, _c| a,
                std::usize::MAX,
                0,
            );
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

        #[test]
        fn for_rsq_and_raq() {
            let ns = vec![3, 4];
            let queriess = vec![
                vec![
                    vec![0, 1, 2, 1],
                    vec![0, 2, 3, 2],
                    vec![0, 3, 3, 3],
                    vec![1, 1, 2],
                    vec![1, 2, 3],
                ],
                vec![vec![1, 1, 4], vec![0, 1, 4, 1], vec![1, 1, 4]],
            ];
            let answers = vec![4, 8, 0, 4];
            let mut index = 0;
            for (n, queries) in ns.into_iter().zip(queriess.into_iter()) {
                let mut tree = DelayedSegmentTree::new(n, |a, b| a + b, 0, |a, b, c| a + b * c);
                for query in queries {
                    if query[0] == 0 {
                        tree.act_in_range(query[1] - 1..query[2], query[3])
                    } else {
                        assert_eq!(tree.find(query[1] - 1..query[2]), answers[index]);
                        index += 1;
                    }
                }
            }
        }
    }
}
